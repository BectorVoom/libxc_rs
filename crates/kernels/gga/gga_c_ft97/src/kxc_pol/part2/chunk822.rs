//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 822/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk822<F: Float>(t14675: F, t446: F, t2876: F, t4051: F, t10248: F, t4057: F, t681: F, t89: F, t14619: F, t14622: F, t14626: F, t14630: F, t14633: F, t14636: F, t14638: F, t14640: F, t14642: F, t14645: F, t14650: F, t14655: F, t14658: F, t14662: F, t14666: F, t14669: F, t14673: F) -> (F, F, F, F, F) {
    let t14676 = t446 * t14675;
    let t14678 = t4051 * t2876;
    let t14679 = t10248 * t14678;
    let t14680 = t446 * t14679;
    let t14683 = t89 * t681 * t4057;
    let t14684 = 2.0 / 9.0 * t14683;
    let t14685 = -4.0 / 9.0 * t14619 + 4.0 / 27.0 * t14622 + t14626 / 18.0 - 2.0 / 9.0 * t14630 + t14633 / 9.0 - t14636 - t14638 + t14640 - t14642 / 27.0 - 5.0 / 81.0 * t14645 - t14650 / 9.0 + t14655 / 27.0 - t14658 + t14662 / 9.0 + t14666 / 18.0 + 2.0 / 9.0 * t14669 - t14673 / 9.0 - t14676 / 3.0 - 2.0 / 9.0 * t14680 - t14684;
    (t14676, t14678, t14680, t14683, t14685)
}
