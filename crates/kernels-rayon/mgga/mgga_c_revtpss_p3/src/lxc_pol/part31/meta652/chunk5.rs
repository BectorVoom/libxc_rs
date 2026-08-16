//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2169/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2169(t1647: f64, t7810: f64, t1078: f64, t1982: f64, t3140: f64, t6343: f64, t100702: f64, t1043: f64, t1089: f64, t1097: f64, t1651: f64, t1652: f64, t1696: f64, t19381: f64, t1986: f64, t20112: f64, t25591: f64, t25695: f64, t25699: f64, t27415: f64, t27422: f64, t27433: f64, t27445: f64, t27621: f64, t27627: f64, t27661: f64, t29747: f64, t29866: f64, t29871: f64, t6235: f64, t6244: f64, t6259: f64, t7102: f64, t7135: f64, t7137: f64, t7145: f64, t7170: f64, t94122: f64, t99675: f64, t999: f64, t99940: f64) -> f64 {
    let t107629 = t1647 * t7810;
    let t107636 = t1982 * t6343 * t3140 * t1078;
    let t107649 = 0.34694512752820797848e1_f64 * t25591 * t7145 * t27422 * t1651 - 0.26020884564615598386e1_f64 * t25699 * t7145 * t7135 * t6244 + 0.34694512752820797848e1_f64 * t25591 * t7145 * t29747 * t999 + 0.34694512752820797848e1_f64 * t27415 * t29866 - 0.65854491829355115987e0_f64 * t25695 * t6259 - 0.65854491829355115987e0_f64 * t7102 * t19381 - 0.8673628188205199462e0_f64 * t27621 * t27627 - 0.4336814094102599731e0_f64 * t1982 * t20112 * t1986 - 0.17347256376410398924e1_f64 * t27661 * t27445 - 0.13170898365871023197e1_f64 * t107629 * t1097 + 0.65854491829355115987e0_f64 * t6235 * t7137 - 0.4336814094102599731e0_f64 * t107636 * t7170 - 0.17347256376410398924e1_f64 * t99675 * t27433 - 0.26020884564615598386e1_f64 * t94122 * t29871 * t1043 * t1089 - 0.13170898365871023197e1_f64 * t99940 * t1652 - 0.13170898365871023197e1_f64 * t100702 * t1696;
    t107649
}
