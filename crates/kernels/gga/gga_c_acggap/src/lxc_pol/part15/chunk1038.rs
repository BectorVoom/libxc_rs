//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1038/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1038<F: Float>(t1844: F, t309: F, t463: F, t157: F, t1658: F, t524: F, t1815: F, t301: F, t9476: F, t1674: F, t1713: F, t8034: F, t10040: F, t1427: F, t1680: F, t2166: F, t24753: F, t24893: F, t32278: F, t36684: F, t36686: F, t36689: F, t38559: F, t38563: F, t5645: F, t567: F, t7297: F, t8040: F, t8372: F, t9448: F, t9460: F, t9469: F) -> (F, F, F, F, F, F, F) {
    let t40733 = t1844 * t309;
    let t40740 = t1844 * t463;
    let t40749 = t1658 * t524 * t157;
    let t40861 = t1815 * t309;
    let t40868 = t1815 * t463;
    let t40955 = t9476 * t301;
    let t40992 = t1674 * t8034 * t1713;
    let t41000 = -t10040 * t2166 * t567 + 12.0 * t1427 * t36686 * t8372 - 2.0 * t1680 * t567 * t9448 - 3.0 * t24753 * t7297 * t8040 - 6.0 * t24893 * t8040 * t8372 + 6.0 * t32278 * t567 * t9469 + 12.0 * t38559 * t7297 * t9460 - 6.0 * t38563 * t7297 * t8040 + 12.0 * t5645 * t8034 * t8372 - t36684 + t36689 + 6.0 * t40992;
    (t40733, t40740, t40749, t40861, t40868, t40955, t41000)
}
