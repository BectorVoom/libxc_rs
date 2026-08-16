//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 251/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk251(t974: f64, t976: f64, t344: f64, t883: f64, t607: f64, t906: f64, t910: f64, t340: f64, t343: f64, t346: f64, t964: f64, t971: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t977 = t974 * t976;
    let t978 = t344 * t883;
    let t979 = t978 * t607;
    let t980 = t977 * t979;
    let t984 = t906 / 6.0_f64 + t910 / 6.0_f64;
    let t985 = t340 * t984;
    let t986 = t985 * t343;
    let t987 = t974 * t986;
    let t990 = -0.22222222222222222222e-2_f64 * t964 * t346 + t971 + 0.27777777777777777777e-3_f64 * t973 * t980 - 0.83333333333333333332e-3_f64 * t973 * t987;
    (t977, t978, t979, t984, t986, t990)
}
