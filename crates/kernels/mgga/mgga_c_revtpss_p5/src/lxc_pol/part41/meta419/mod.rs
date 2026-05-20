//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1473;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta419<F: Float>(t31303: F, t31326: F, t3: F, t2178: F, t670: F, t1518: F, t31117: F, t4292: F, t8295: F, t116: F, t8362: F, t117: F, t31292: F, param_d: F, t1459: F, t1461: F, t1916: F, t1918: F, t2187: F, t2189: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t8289: F, t8296: F, t8299: F, t8377: F, t8383: F, t8386: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t31328, t31329, t31340, t31358, t31359, t31362, t31365, t31370, t31371, t31374) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1473::<F>(t31303, t31326, t3, t2178, t670, t1518, t31117, t4292, t8295, t116, t8362, t117, t31292, param_d);
        let t31377 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1474::<F>(t1459, t1461, t1916, t1918, t2187, t2189, t31340, t31359, t31362, t31365, t31371, t31374, t572, t573, t5795, t5802, t5805, t8289, t8296, t8299, t8377, t8383, t8386);
    (t31328, t31329, t31340, t31358, t31359, t31362, t31365, t31370, t31371, t31374, t31377)
}
