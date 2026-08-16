//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2200/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2200(t1976: f64, t4743: f64, t1695: f64, t988: f64, t27543: f64, t342: f64, t1043: f64, t1089: f64, t1096: f64, t1097: f64, t15579: f64, t16328: f64, t1985: f64, t25605: f64, t25629: f64, t25695: f64, t25699: f64, t27411: f64, t27440: f64, t27444: f64, t27651: f64, t27679: f64, t27691: f64, t3059: f64, t3066: f64, t4941: f64, t4975: f64, t5015: f64, t7102: f64, t7135: f64, t7144: f64, t7145: f64, t7159: f64, t7160: f64, t7810: f64, t7822: f64, t93485: f64, t93497: f64, t93498: f64, t93921: f64, t94122: f64, t999: f64) -> f64 {
    let t99629 = t4743 * t1976;
    let t99638 = t1695 * t988;
    let t99666 = t342 * t27543;
    let t99673 = 0.17347256376410398924e1_f64 * t7159 * t7160 * t7135 * t5015 - 0.26020884564615598386e1_f64 * t25699 * t7145 * t7810 * t3059 + 0.13170898365871023197e1_f64 * t7102 * t16328 - 0.13170898365871023197e1_f64 * t99629 * t1097 + 0.65854491829355115987e0_f64 * t7102 * t15579 - 0.34694512752820797848e1_f64 * t93497 * t27651 * t4975 * t3066 - 0.69389025505641595696e1_f64 * t93921 * t1985 * t99638 * t999 - 0.52041769129231196772e1_f64 * t94122 * t27691 * t93498 + 0.8673628188205199462e0_f64 * t93485 * t7822 + 0.34694512752820797848e1_f64 * t7144 * t7160 * t27444 * t1096 + 0.17347256376410398924e1_f64 * t25605 * t27411 * t1043 * t1089 - 0.17347256376410398924e1_f64 * t25629 * t27679 * t1043 * t1089 + 0.17347256376410398924e1_f64 * t25605 * t27440 * t1043 * t1089 + 0.13170898365871023197e1_f64 * t25695 * t4941 - 0.13170898365871023197e1_f64 * t99666 * t1097 - 0.17347256376410398924e1_f64 * t25629 * t27444 * t1043 * t1089;
    t99673
}
