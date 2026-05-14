//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1100/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1100<F: Float>(t14997: F, t15022: F, t15044: F, t15069: F, t2430: F, t4542: F, t10596: F, t10604: F, t10611: F, t14436: F, t14442: F, t14443: F, t14444: F, t14468: F, t14615: F, t14618: F, t14620: F, t14621: F, t14624: F, t14626: F, t14628: F, t14629: F, t1940: F, t198: F, t207: F, t2404: F, t2408: F, t4433: F, t4541: F, t765: F, t892: F, t9542: F) -> (F,) {
    let t15071 = t14997 + t15022 + t15044 + t15069;
    let t15078 = t4542 * t2430;
    let t15081 = t15071 * t198 * t207 * t892 + 2.0 * t14436 * t1940 * t2408 + 3.0 * t14468 * t198 * t765 + 12.0 * t2404 * t4433 * t4541 + 6.0 * t15078 * t4541 - t10596 - t10604 - t10611 + t14442 - t14443 - t14444 + t14615 - t14618 + t14620 + t14621 + t14624 + t14626 + t14628 + t14629 + t9542;
    (t15081,)
}
