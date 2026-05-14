//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 736/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk736<F: Float>(t1079: F, t5015: F, t1000: F, t1073: F, t1076: F, t1097: F, t1647: F, t1652: F, t1680: F, t1696: F, t3047: F, t3052: F, t3058: F, t3063: F, t3264: F, t342: F, t386: F, t4743: F, t4747: F, t4752: F, t4758: F, t4764: F, t4773: F, t4778: F, t4932: F, t4935: F, t4941: F, t4947: F, t989: F, t995: F) -> (F, F) {
    let t5016 = t1079 * t5015;
    let t5019 = 0.65854491829355115987e0 * t4743 * t386 - 0.65854491829355115987e0 * t4747 * t1000 + 0.65854491829355115987e0 * t1647 * t1073 - 0.65854491829355115987e0 * t4752 * t1097 - 0.65854491829355115987e0 * t3047 * t1652 + 0.13170898365871023197e1 * t3058 * t4758 - 0.65854491829355115987e0 * t3063 * t1652 + 0.65854491829355115987e0 * t995 * t4764 - 0.65854491829355115987e0 * t995 * t4773 + 0.65854491829355115987e0 * t989 * t1680 - 0.65854491829355115987e0 * t4778 * t1000 + 0.65854491829355115987e0 * t342 * t4932 - 0.65854491829355115987e0 * t4935 * t1097 - 0.65854491829355115987e0 * t3052 * t1696 + 0.65854491829355115987e0 * t995 * t4941 - 0.65854491829355115987e0 * t3264 * t1696 + 0.13170898365871023197e1 * t1076 * t4947 - 0.65854491829355115987e0 * t1076 * t5016;
    (t5016, t5019)
}
