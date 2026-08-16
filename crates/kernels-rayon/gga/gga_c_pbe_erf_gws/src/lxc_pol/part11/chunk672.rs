//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 672/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk672(t1062: f64, t1903: f64, t2519: f64, t713: f64, t1009: f64, t4991: f64, t587: f64, t1022: f64, t1697: f64, t197: f64, t5283: f64, t1802: f64, t1885: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7541 = t1062 * t1903;
    let t7573 = t2519 * t713;
    let t7579 = t4991 * t1009;
    let t7580 = t587 * t7579;
    let t7651 = t1022 * t1697;
    let t7669 = t5283 * t197;
    let t7703 = t1885 * t1802;
    (t7541, t7573, t7579, t7580, t7651, t7669, t7703)
}
