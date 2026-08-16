//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1291/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1291(t11120: f64, t3140: f64, t1035: f64, t1983: f64, t1043: f64, t1089: f64, t11122: f64, t11174: f64, t11202: f64, t12132: f64, t1976: f64, t25465: f64, t25466: f64, t25473: f64, t25476: f64, t25483: f64, t25586: f64, t25601: f64, t25611: f64, t25613: f64, t25699: f64, t27669: f64, t3059: f64, t4976: f64, t7102: f64, t7135: f64, t7144: f64, t7145: f64, t7147: f64, t7159: f64, t7162: f64, t93892: f64, t93959: f64, t93963: f64, t93968: f64, t93974: f64, t93983: f64, t93984: f64, t93989: f64, t93994: f64, t94005: f64, t988: f64) -> f64 {
    let t94014 = t3140 * t11120;
    let t94016 = t1983 * t94014 * t1035;
    let t94021 = -0.26020884564615598386e1_f64 * t93959 * t7147 + 0.52041769129231196772e1_f64 * t93963 * t25613 + 0.10408353825846239354e2_f64 * t25476 * t25601 + 0.10408353825846239354e2_f64 * t7159 * t93968 * t1976 * t11122 + 0.52041769129231196772e1_f64 * t25611 * t93974 * t4976 + 0.26020884564615598386e1_f64 * t25611 * t25483 * t1043 * t1089 + 0.52041769129231196772e1_f64 * t93983 * t93892 * t93984 - 0.26020884564615598386e1_f64 * t27669 * t93989 * t12132 + 0.10408353825846239354e2_f64 * t93994 * t7145 * t1976 * t11202 - 0.78062653693846795158e1_f64 * t25699 * t7145 * t7135 * t3059 - 0.78062653693846795158e1_f64 * t25473 * t25466 + 0.26020884564615598386e1_f64 * t94005 * t7162 - 0.26020884564615598386e1_f64 * t7144 * t7145 * t25586 * t988 - 0.65854491829355115987e0_f64 * t7102 * t11174 - 0.78062653693846795158e1_f64 * t94016 * t25465 * t1043 * t1089;
    t94021
}
