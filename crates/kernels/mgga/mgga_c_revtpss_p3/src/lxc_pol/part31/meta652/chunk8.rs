//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2172/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2172<F: Float>(t29894: F, t3336: F, t100802: F, t100806: F, t106684: F, t106738: F, t106786: F, t106834: F, t107206: F, t107257: F, t107305: F, t107354: F, t107405: F, t107457: F, t107509: F, t107557: F, t107603: F, t107649: F, t107691: F, t107733: F, t1100: F, t1102: F, t1699: F, t198: F, t20230: F, t25709: F, t25713: F, t27712: F, t27717: F, t336: F, t5019: F, t5023: F, t6396: F, t6400: F, t7181: F, t94142: F, t94149: F) -> F {
    let t107741 = t29894 * t3336;
    let t107772 = t198 * t336 * (t106684 + t106738 + t106786 + t106834 + t107206 + t107257 + t107305 + t107354 + t107405 + t107457 + t107509 + t107557 + t107603 + t107649 + t107691 + t107733) * t1102 - t5023 * t107741 * t1100 - F::new(2.0) * t5023 * t100802 * t1699 + F::new(4.0) * t5023 * t100806 * t27717 - F::new(2.0) * t5023 * t27712 * t5019 + F::new(2.0) * t5023 * t94142 * t6400 - F::new(6.0) * t5023 * t94149 * t6400 * t1100 + F::new(4.0) * t5023 * t25713 * t1699 * t5019 - t5023 * t25709 * t6396 + F::new(2.0) * t5023 * t25713 * t6396 * t1100 - t5023 * t7181 * t20230;
    t107772
}
