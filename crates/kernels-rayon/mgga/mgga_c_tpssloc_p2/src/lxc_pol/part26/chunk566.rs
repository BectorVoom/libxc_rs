//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 566/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk566(t3040: f64, t3131: f64, t1021: f64, t248: f64, t135: f64, t999: f64, t973: f64, t2250: f64, t998: f64, t974: f64, t2770: f64, t2978: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3132 = t3040 * t3131;
    let t3134 = t248 * t1021 * t3132;
    let t3139 = t135 * t999;
    let t3140 = t973 * t3139;
    let t3142 = t998 * t2250;
    let t3143 = t974 * t3142;
    let t3146 = t2978 * t2770;
    (t3132, t3134, t3139, t3140, t3142, t3143, t3146)
}
