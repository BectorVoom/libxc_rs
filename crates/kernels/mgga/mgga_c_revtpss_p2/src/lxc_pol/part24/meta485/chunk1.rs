//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1478/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1478<F: Float>(t1244: F, t3594: F, t71691: F, t17628: F, t5373: F, t3655: F, t6595: F, t1222: F, t6658: F, t697: F, t6662: F, t1209: F, t1284: F, t6695: F) -> (F, F, F, F, F, F) {
    let t71699 = t3594 * t1244 * t71691;
    let t71718 = t5373 * t17628;
    let t71744 = t6595 * t3655;
    let t71928 = t1222 * t697 * t6658;
    let t71931 = t1222 * t697 * t6662;
    let t72267 = t1209 * t1284 * t6695;
    (t71699, t71718, t71744, t71928, t71931, t72267)
}
