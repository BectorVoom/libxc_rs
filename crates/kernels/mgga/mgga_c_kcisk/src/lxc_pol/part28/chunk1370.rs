//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1370/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1370<F: Float>(t32965: F, t415: F, t8677: F, t1799: F, t22988: F, t34159: F, t35104: F, t5074: F, t121021: F, t9649: F, t34097: F, t34122: F, t1772: F, t23922: F, t648: F, t71232: F) -> (F, F, F, F, F, F, F) {
    let t121586 = t415 * t32965 * t8677;
    let t121589 = t1799 * t34159 * t22988;
    let t121592 = t5074 * t35104;
    let t121594 = t9649 * t121021;
    let t121597 = t34122 * t34097;
    let t121600 = t23922 * t648 * t1772;
    let t121606 = t71232 * t648 * t1772;
    (t121586, t121589, t121592, t121594, t121597, t121600, t121606)
}
