//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1342/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1342(t16237: f64, t225: f64, t385: f64, t1096: f64, t4772: f64, t1079: f64, t1651: f64, t3269: f64, t3270: f64, t5015: f64, t1073: f64, t1076: f64, t11190: f64, t11224: f64, t15579: f64, t15886: f64, t1647: f64, t1652: f64, t3047: f64, t3052: f64, t3063: f64, t3261: f64, t342: f64, t386: f64, t4743: f64, t4758: f64, t4764: f64, t4932: f64, t4941: f64, t4947: f64, t989: f64, t995: f64) -> f64 {
    let t16239 = t16237 * t225 * t385;
    let t16242 = t4772 * t1096;
    let t16243 = t1079 * t16242;
    let t16249 = t3269 * t1651 * t3270;
    let t16254 = t5015 * t1096;
    let t16255 = t3269 * t16254;
    let t16272 = 0.65854491829355115987e0_f64 * t995 * t15579 + 0.65854491829355115987e0_f64 * t342 * t16239 + 0.13170898365871023197e1_f64 * t995 * t16243 + 0.13170898365871023197e1_f64 * t3047 * t4941 - 0.13170898365871023197e1_f64 * t995 * t16249 + 0.26341796731742046394e1_f64 * t3052 * t4947 + 0.26341796731742046394e1_f64 * t1076 * t16255 + 0.13170898365871023197e1_f64 * t989 * t4932 + 0.13170898365871023197e1_f64 * t3063 * t4764 - 0.65854491829355115987e0_f64 * t11190 * t1652 + 0.26341796731742046394e1_f64 * t11224 * t4758 + 0.65854491829355115987e0_f64 * t1647 * t3261 + 0.65854491829355115987e0_f64 * t15886 * t386 + 0.13170898365871023197e1_f64 * t4743 * t1073;
    t16272
}
