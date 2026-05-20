//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3014/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3014<F: Float>(t4423: F, t853: F, t2661: F, t2662: F, t2749: F, t14718: F, t14872: F, t10777: F, t10779: F, t1548: F, t2754: F, t14671: F, t14686: F, t14931: F, t2724: F) -> (F, F, F, F, F) {
    let t50583 = t853 * t4423;
    let t50586 = t2661 * t2662 * t50583 * t2749;
    let t50590 = t2661 * t2662 * t14718 * t14872;
    let t50594 = t10777 * t10779 * t1548 * t2754;
    let t50598 = t14931 * t14686 * t14671 * t2724;
    (t50583, t50586, t50590, t50594, t50598)
}
