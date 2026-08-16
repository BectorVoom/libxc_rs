//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1021/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1021<F: Float>(t5: F, t117487: F, t117528: F, t112: F, t115783: F, t115785: F, t115788: F, t115790: F, t115792: F, t115796: F, t115802: F, t115813: F, t115815: F, t115817: F, t115819: F, t117445: F, t8446: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t117530 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t117487 + t117528);
    let t117531 = t117530 * t112;
    let t117532 = t8446 + F::cast_from(2.0_f64) * t117445 + t117531 + t115783 + t115785 + t115788 + t115790 + t115792 + t115796 + t115802 + t115813 + t115815 + t115817 + t115819;
    (t117531, t117532)
}
