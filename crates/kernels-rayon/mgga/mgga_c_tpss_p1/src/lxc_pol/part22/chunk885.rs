//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 885/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk885(t2257: f64, t704: f64, t172: f64, t2274: f64, t182: f64, t2209: f64, t177: f64, t2214: f64, t7813: f64, t7821: f64, t7824: f64, t7827: f64, t7830: f64, t7834: f64, t7836: f64, t7838: f64, t7841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7850 = t2257 * t704;
    let t7852 = 1.0_f64 / t2274 / t172;
    let t7853 = t7850 * t7852;
    let t7857 = 1.0_f64 / t2209 / t182;
    let t7858 = t177 * t7857;
    let t7859 = t7813 * t2214;
    let t7870 = -0.34523333333333333333e1_f64 * t7821 + 0.23015555555555555556e1_f64 * t7824 - 0.26851481481481481482e1_f64 * t7827 - 0.93932222222222222223e0_f64 * t7830 + 0.73355e-1_f64 * t7834 - 0.14671e0_f64 * t7836 - 0.17116166666666666667e0_f64 * t7838 - 0.36793333333333333333e0_f64 * t7841;
    (t7850, t7853, t7857, t7858, t7859, t7870)
}
