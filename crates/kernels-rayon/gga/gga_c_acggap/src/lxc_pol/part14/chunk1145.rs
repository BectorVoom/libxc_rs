//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1145/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1145(t2288: f64, t8901: f64, t15386: f64, t31195: f64, t2001: f64, t6076: f64, t1998: f64, t6081: f64, t1856: f64, t7614: f64, t35259: f64, t35261: f64, t37464: f64, t39797: f64, t39802: f64, t39807: f64, t39809: f64, t39811: f64, t39813: f64, t39815: f64, t39817: f64, t39819: f64, t39822: f64, t39825: f64) -> (f64, f64) {
    let t39827 = t2288 * t8901;
    let t39829 = t31195 * t15386 * t39827;
    let t39831 = t2001 * t6076;
    let t39833 = t1998 * t6081;
    let t39835 = t7614 * t1856;
    let t39837 = t35259 + 0.64311027177104605458e-3_f64 * t39797 - t35261 - 0.31448092289604152068e-2_f64 * t39802 - 0.18868855373762491241e-1_f64 * t39807 - 0.17149607247227894789e-2_f64 * t39809 - 0.68598428988911579156e-2_f64 * t39811 + 0.34299214494455789578e-2_f64 * t39813 - 0.51448821741683684367e-2_f64 * t39815 + 0.17149607247227894789e-2_f64 * t39817 + 0.25724410870841842183e-2_f64 * t39819 + t39822 / 24.0_f64 + t39825 / 192.0_f64 + 0.47172138434406228102e-2_f64 * t39829 - 0.17149607247227894789e-2_f64 * t39831 + 0.85748036236139473944e-3_f64 * t39833 + 0.16006300097412701803e-1_f64 * t39835 - t37464;
    (t39827, t39837)
}
