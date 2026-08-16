//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1178/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1178(t1043: f64, t1089: f64, t7168: f64, t1984: f64, t359: f64, t7135: f64, t1000: f64, t1097: f64, t1978: f64, t1983: f64, t1986: f64, t342: f64, t7102: f64, t7137: f64, t7140: f64, t7144: f64, t7147: f64, t7151: f64, t7153: f64, t7156: f64, t7159: f64, t7162: f64, t7167: f64, t989: f64) -> (f64, f64, f64) {
    let t7170 = t7168 * t1043 * t1089;
    let t7174 = t1984 * t359 * t7135;
    let t7177 = 0.65854491829355115987e0_f64 * t989 * t1978 - 0.65854491829355115987e0_f64 * t7102 * t1000 + 0.65854491829355115987e0_f64 * t342 * t7137 - 0.65854491829355115987e0_f64 * t7140 * t1097 - 0.8673628188205199462e0_f64 * t7144 * t7147 + 0.8673628188205199462e0_f64 * t7151 * t7153 - 0.4336814094102599731e0_f64 * t7156 * t1986 + 0.8673628188205199462e0_f64 * t7159 * t7162 - 0.4336814094102599731e0_f64 * t7167 * t7170 - 0.4336814094102599731e0_f64 * t1983 * t7174;
    (t7170, t7174, t7177)
}
