//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2749/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2749<F: Float>(t14738: F, t2741: F, t10845: F, t14732: F, t4423: F, t853: F, t2661: F, t2662: F, t2749: F, t14718: F, t14872: F, t10777: F, t10779: F, t1548: F, t2754: F) -> (F, F, F, F, F) {
    let t50579 = t2741 * t14738;
    let t50581 = t10845 * t14732;
    let t50582 = F::cast_from(0.40656002247428262579e-3_f64) * t50581;
    let t50583 = t853 * t4423;
    let t50586 = t2661 * t2662 * t50583 * t2749;
    let t50590 = t2661 * t2662 * t14718 * t14872;
    let t50594 = t10777 * t10779 * t1548 * t2754;
    (t50579, t50582, t50586, t50590, t50594)
}
