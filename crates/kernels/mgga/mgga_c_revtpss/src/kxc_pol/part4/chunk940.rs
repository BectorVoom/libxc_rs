//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 940/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk940<F: Float>(t2665: F, t9775: F, t2681: F, t820: F, t849: F, t857: F, t240: F, t2719: F, t243: F, t2722: F, t2723: F, t2661: F, t231: F, t2662: F, t221: F, t2430: F, t2675: F) -> (F, F, F, F, F, F, F) {
    let t10719 = t9775 * t2665;
    let t10722 = t820 * t849 * t2681;
    let t10723 = t10722 * t857;
    let t10726 = t2719 * t240;
    let t10727 = t243 * t2722;
    let t10728 = t10727 * t2723;
    let t10729 = t10726 * t10728;
    let t10730 = t2661 * t10729;
    let t10732 = t10727 * t231;
    let t10733 = t2662 * t10732;
    let t10734 = t2661 * t10733;
    let t10741 = t2675 * t221 * t2430;
    (t10719, t10722, t10723, t10726, t10730, t10734, t10741)
}
