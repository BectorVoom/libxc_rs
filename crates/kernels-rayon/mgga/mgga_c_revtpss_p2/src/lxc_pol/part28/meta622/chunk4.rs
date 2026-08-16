//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2205/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2205(t988: f64, t999: f64, t73: f64, t99729: f64, t1647: f64, t7135: f64, t1078: f64, t1982: f64, t3140: f64, t4930: f64, t1089: f64, t1097: f64, t15885: f64, t1976: f64, t25461: f64, t25464: f64, t25470: f64, t25588: f64, t25629: f64, t25681: f64, t25699: f64, t27426: f64, t27568: f64, t27576: f64, t27587: f64, t27609: f64, t27651: f64, t27652: f64, t3059: f64, t3075: f64, t3076: f64, t3270: f64, t4866: f64, t4975: f64, t7144: f64, t7145: f64, t7151: f64, t7160: f64, t7167: f64, t7170: f64, t7174: f64, t7821: f64, t7825: f64, t7828: f64, t7829: f64, t93502: f64, t94005: f64) -> (f64, f64) {
    let t99858 = t988 * t999;
    let t99877 = t99729 * t73;
    let t99881 = t1647 * t7135;
    let t99886 = t1982 * t4930 * t3140 * t1078;
    let t99901 = 0.52041769129231196772e1_f64 * t25699 * t7160 * t7828 * t3059 + 0.34694512752820797848e1_f64 * t7144 * t7160 * t27426 * t988 - 0.4336814094102599731e0_f64 * t7825 * t25588 + 0.34694512752820797848e1_f64 * t93502 * t27651 * t4975 * t99858 + 0.52041769129231196772e1_f64 * t7151 * t25464 * t7821 * t3270 + 0.8673628188205199462e0_f64 * t94005 * t7829 - 0.65854491829355115987e0_f64 * t27568 * t3076 - 0.8673628188205199462e0_f64 * t27587 * t7174 + 0.17347256376410398924e1_f64 * t27609 * t25470 - 0.34694512752820797848e1_f64 * t25461 * t27576 - 0.17347256376410398924e1_f64 * t25629 * t99877 * t27652 - 0.13170898365871023197e1_f64 * t99881 * t1097 - 0.8673628188205199462e0_f64 * t99886 * t7170 - 0.8673628188205199462e0_f64 * t7167 * t25681 * t4866 * t1089 - 0.8673628188205199462e0_f64 * t7144 * t7145 * t1976 * t15885 - 0.17347256376410398924e1_f64 * t7151 * t7160 * t7828 * t3075;
    (t99877, t99901)
}
