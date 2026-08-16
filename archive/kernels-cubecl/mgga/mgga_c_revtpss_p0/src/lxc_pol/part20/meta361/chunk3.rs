//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1314/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1314<F: Float>(t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39738: F, t39741: F, t39744: F, t39747: F, t39750: F, t215: F, t2581: F, t2585: F, t268: F) -> (F, F) {
    let t39751 = t39520 - t39528 + t39531 + t39534 + t39537 - t39540 + t39738 + t39741 + t39744 + t39747 + t39750;
    let t39756 = F::cast_from(0.22911460125803964958e1_f64) * t268 * t215 * t2581 * t2585;
    (t39751, t39756)
}
