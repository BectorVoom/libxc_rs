//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1622/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1622<F: Float>(t16237: F, t225: F, t385: F, t1096: F, t4772: F, t1079: F, t1651: F, t3269: F, t3270: F, t5015: F, t1073: F, t1076: F, t11190: F, t11224: F, t15579: F, t15886: F, t1647: F, t1652: F, t3047: F, t3052: F, t3063: F, t3261: F, t342: F, t386: F, t4743: F, t4758: F, t4764: F, t4932: F, t4941: F, t4947: F, t989: F, t995: F) -> F {
    let t16239 = t16237 * t225 * t385;
    let t16242 = t4772 * t1096;
    let t16243 = t1079 * t16242;
    let t16249 = t3269 * t1651 * t3270;
    let t16254 = t5015 * t1096;
    let t16255 = t3269 * t16254;
    let t16272 = F::cast_from(0.65854491829355115987e0_f64) * t995 * t15579 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t16239 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t16243 + F::cast_from(0.13170898365871023197e1_f64) * t3047 * t4941 - F::cast_from(0.13170898365871023197e1_f64) * t995 * t16249 + F::cast_from(0.26341796731742046394e1_f64) * t3052 * t4947 + F::cast_from(0.26341796731742046394e1_f64) * t1076 * t16255 + F::cast_from(0.13170898365871023197e1_f64) * t989 * t4932 + F::cast_from(0.13170898365871023197e1_f64) * t3063 * t4764 - F::cast_from(0.65854491829355115987e0_f64) * t11190 * t1652 + F::cast_from(0.26341796731742046394e1_f64) * t11224 * t4758 + F::cast_from(0.65854491829355115987e0_f64) * t1647 * t3261 + F::cast_from(0.65854491829355115987e0_f64) * t15886 * t386 + F::cast_from(0.13170898365871023197e1_f64) * t4743 * t1073;
    t16272
}
