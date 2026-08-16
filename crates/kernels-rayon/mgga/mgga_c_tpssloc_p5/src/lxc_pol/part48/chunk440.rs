//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 440/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk440(t3139: f64, t973: f64, t2250: f64, t998: f64, t974: f64, t2770: f64, t2978: f64, t2244: f64, t2775: f64, t976: f64, t1005: f64, t1036: f64) -> (f64, f64, f64, f64, f64) {
    let t3140 = t973 * t3139;
    let t3142 = t998 * t2250;
    let t3143 = t974 * t3142;
    let t3146 = t2978 * t2770;
    let t3147 = t3146 * t2244;
    let t3148 = t974 * t3147;
    let t3151 = t976 * t2775;
    let t3152 = t3151 * t2244;
    let t3153 = t974 * t3152;
    let t3156 = t1005 * t1036;
    (t3140, t3143, t3148, t3153, t3156)
}
