//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1288/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1288<F: Float>(t3268: F, t7143: F, t3057: F, t25460: F, t25698: F, t1035: F, t25586: F, t1976: F, t3133: F, t73: F, t1043: F, t1089: F, t11123: F, t1983: F, t1984: F, t1985: F, t25476: F, t25487: F, t25611: F, t25621: F, t25629: F, t25640: F, t25648: F, t25658: F, t25681: F, t25687: F, t25701: F, t27415: F, t27652: F, t3066: F, t3271: F, t3325: F, t3326: F, t359: F, t4976: F, t7135: F, t7140: F, t7159: F, t7160: F, t7167: F, t7174: F, t93827: F, t988: F) -> (F, F) {
    let t93920 = t7143 * t3268;
    let t93921 = t3057 * t93920;
    let t93928 = t25698 * t25460;
    let t93939 = t1035 * t25586;
    let t93945 = t1976 * t3133 * t73;
    let t93958 = F::new(0.26020884564615598386e1) * t7159 * t7160 * t7135 * t3325 - F::new(0.26020884564615598386e1) * t27415 * t25621 - F::new(0.52041769129231196772e1) * t25476 * t25648 - F::new(0.13010442282307799193e1) * t25640 * t25687 - F::new(0.20816707651692478709e2) * t93921 * t1985 * t3066 * t988 + F::new(0.39512695097613069591e1) * t25658 * t3271 - F::new(0.78062653693846795158e1) * t93928 * t25701 - F::new(0.19756347548806534796e1) * t25658 * t3326 - F::new(0.39512695097613069591e1) * t7140 * t11123 - F::new(0.13010442282307799193e1) * t7167 * t25681 * t3133 * t1089 - F::new(0.13010442282307799193e1) * t7167 * t93939 * t1043 * t1089 - F::new(0.26020884564615598386e1) * t25629 * t93945 * t27652 + F::new(0.26020884564615598386e1) * t25611 * t93945 * t4976 - F::new(0.13010442282307799193e1) * t25487 * t7174 - F::new(0.4336814094102599731e0) * t1983 * t1984 * t359 * t93827;
    (t93945, t93958)
}
