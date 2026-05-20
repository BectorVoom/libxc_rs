//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1156/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1156<F: Float>(t342: F, t4930: F, t1071: F, t1647: F, t378: F, t4743: F, t1678: F, t989: F, t15654: F, t1086: F, t359: F, t3286: F, t4746: F) -> (F, F, F, F, F, F, F, F) {
    let t16333 = t342 * t4930;
    let t16340 = t1647 * t1071;
    let t16362 = t4743 * t378;
    let t16371 = t989 * t1678;
    let t16374 = t15654 * t378;
    let t16381 = t4743 * t1086;
    let t16449 = t359 * t4930;
    let t16502 = t4746 * t3286;
    (t16333, t16340, t16362, t16371, t16374, t16381, t16449, t16502)
}
