//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1288/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1288(t3268: f64, t7143: f64, t3057: f64, t25460: f64, t25698: f64, t1035: f64, t25586: f64, t1976: f64, t3133: f64, t73: f64, t1043: f64, t1089: f64, t11123: f64, t1983: f64, t1984: f64, t1985: f64, t25476: f64, t25487: f64, t25611: f64, t25621: f64, t25629: f64, t25640: f64, t25648: f64, t25658: f64, t25681: f64, t25687: f64, t25701: f64, t27415: f64, t27652: f64, t3066: f64, t3271: f64, t3325: f64, t3326: f64, t359: f64, t4976: f64, t7135: f64, t7140: f64, t7159: f64, t7160: f64, t7167: f64, t7174: f64, t93827: f64, t988: f64) -> (f64, f64) {
    let t93920 = t7143 * t3268;
    let t93921 = t3057 * t93920;
    let t93928 = t25698 * t25460;
    let t93939 = t1035 * t25586;
    let t93945 = t1976 * t3133 * t73;
    let t93958 = 0.26020884564615598386e1_f64 * t7159 * t7160 * t7135 * t3325 - 0.26020884564615598386e1_f64 * t27415 * t25621 - 0.52041769129231196772e1_f64 * t25476 * t25648 - 0.13010442282307799193e1_f64 * t25640 * t25687 - 0.20816707651692478709e2_f64 * t93921 * t1985 * t3066 * t988 + 0.39512695097613069591e1_f64 * t25658 * t3271 - 0.78062653693846795158e1_f64 * t93928 * t25701 - 0.19756347548806534796e1_f64 * t25658 * t3326 - 0.39512695097613069591e1_f64 * t7140 * t11123 - 0.13010442282307799193e1_f64 * t7167 * t25681 * t3133 * t1089 - 0.13010442282307799193e1_f64 * t7167 * t93939 * t1043 * t1089 - 0.26020884564615598386e1_f64 * t25629 * t93945 * t27652 + 0.26020884564615598386e1_f64 * t25611 * t93945 * t4976 - 0.13010442282307799193e1_f64 * t25487 * t7174 - 0.4336814094102599731e0_f64 * t1983 * t1984 * t359 * t93827;
    (t93945, t93958)
}
