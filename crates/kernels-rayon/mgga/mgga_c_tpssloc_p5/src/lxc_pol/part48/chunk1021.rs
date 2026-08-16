//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1021/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1021(t5: f64, t117487: f64, t117528: f64, t112: f64, t115783: f64, t115785: f64, t115788: f64, t115790: f64, t115792: f64, t115796: f64, t115802: f64, t115813: f64, t115815: f64, t115817: f64, t115819: f64, t117445: f64, t8446: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t117530 = piecewise3(t8, 0.0_f64, t117487 + t117528);
    let t117531 = t117530 * t112;
    let t117532 = t8446 + 2.0_f64 * t117445 + t117531 + t115783 + t115785 + t115788 + t115790 + t115792 + t115796 + t115802 + t115813 + t115815 + t115817 + t115819;
    (t117531, t117532)
}
