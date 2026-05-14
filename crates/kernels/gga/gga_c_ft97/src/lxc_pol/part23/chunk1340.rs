//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1340/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1340<F: Float>(t1091: F, t113238: F, t2665: F, t6317: F, t28735: F, t28736: F, t28746: F, t840: F, t24980: F, t24981: F, t31582: F, t684: F, t125866: F, t24976: F, t28755: F, t113061: F, t126736: F, t126740: F, t126744: F, t126749: F, t126753: F, t126757: F, t126761: F) -> (F, F, F, F, F) {
    let t126765 = t6317 * t2665 * t113238 * t1091;
    let t126769 = t28735 * t840 * t28746 * t28736;
    let t126773 = t24980 * t24981 * t31582 * t684;
    let t126776 = t28755 * t24976 * t125866;
    let t126778 = -t126736 / 2.0 - 3.0 * t126740 - t126744 / 3.0 + 3.0 / 4.0 * t126749 - 3.0 / 4.0 * t126753 + 12.0 * t126757 - 6.0 * t126761 + t126765 / 3.0 - 3.0 / 4.0 * t126769 - t126773 / 12.0 + t113061 + 4.0 / 3.0 * t126776;
    (t126765, t126769, t126773, t126776, t126778)
}
