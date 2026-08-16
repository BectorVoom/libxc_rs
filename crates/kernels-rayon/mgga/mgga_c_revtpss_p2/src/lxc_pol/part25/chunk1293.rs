//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1293/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1293(t25625: f64, t7143: f64, t1043: f64, t1089: f64, t11173: f64, t11184: f64, t11203: f64, t1976: f64, t25470: f64, t25473: f64, t25476: f64, t25479: f64, t25591: f64, t25605: f64, t25616: f64, t25621: f64, t25631: f64, t25700: f64, t27664: f64, t3060: f64, t3325: f64, t7102: f64, t7144: f64, t7145: f64, t7146: f64, t7151: f64, t7152: f64, t7160: f64, t7162: f64, t93892: f64, t93945: f64, t94023: f64, t94026: f64, t94042: f64, t94053: f64, t94063: f64, t94064: f64, t988: f64) -> f64 {
    let t94068 = t25625 * t7143;
    let t94075 = 0.39512695097613069591e1_f64 * t94023 * t3060 - 0.39512695097613069591e1_f64 * t94026 * t11203 + 0.8673628188205199462e0_f64 * t7151 * t7145 * t1976 * t11173 + 0.52041769129231196772e1_f64 * t7144 * t7160 * t7146 * t3325 - 0.26020884564615598386e1_f64 * t25476 * t25621 + 0.26020884564615598386e1_f64 * t25605 * t93945 * t27664 - 0.52041769129231196772e1_f64 * t94042 * t25631 + 0.10408353825846239354e2_f64 * t25591 * t7145 * t25616 * t988 - 0.52041769129231196772e1_f64 * t7151 * t7160 * t7152 * t3325 - 0.15612530738769359031e2_f64 * t94053 * t7145 * t25700 * t988 + 0.19756347548806534796e1_f64 * t7102 * t11184 + 0.52041769129231196772e1_f64 * t25473 * t25470 - 0.26020884564615598386e1_f64 * t94063 * t93892 * t94064 + 0.52041769129231196772e1_f64 * t94068 * t7162 + 0.26020884564615598386e1_f64 * t25605 * t25479 * t1043 * t1089;
    t94075
}
