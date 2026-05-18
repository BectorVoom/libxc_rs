//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1181/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1181<F: Float>(t14622: F, t4401: F, t2414: F, t4311: F, t10428: F, t1522: F, t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14442: F, t14443: F, t14444: F, t14615: F, t14618: F, t14620: F, t14621: F, t9542: F) -> (F, F, F, F, F) {
    let t14624 = F::new(12.0) * t4401 * t14622;
    let t14626 = F::new(4.0) * t4311 * t2414;
    let t14628 = F::new(4.0) * t10428 * t1522;
    let t14629 = F::new(8.0) * t10613;
    let t14630 = t10592 + t14442 - t14443 - t10596 - t14444 - t10604 + t9542 + t14615 - t14618 + t14620 + t14621 + t14624 - t10611 + t14626 + t14628 + t14629;
    (t14624, t14626, t14628, t14629, t14630)
}
