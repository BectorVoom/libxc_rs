//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 774/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk774(t2988: f64, t2990: f64, t2775: f64, t344: f64, t2244: f64, t977: f64, t2250: f64, t978: f64, t2822: f64, t2824: f64, t2828: f64, t2831: f64, t2834: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2991 = t2988 * t2990;
    let t2994 = t344 * t2775;
    let t2995 = t2994 * t2244;
    let t2996 = t977 * t2995;
    let t2999 = t978 * t2250;
    let t3000 = t977 * t2999;
    let t3003 = 5.0_f64 / 18.0_f64 * t2822;
    let t3008 = -t3003 - 2.0_f64 / 9.0_f64 * t2824 + t2828 / 18.0_f64 - t2831 / 3.0_f64 + t2834 / 6.0_f64;
    (t2991, t2995, t2996, t2999, t3000, t3003, t3008)
}
