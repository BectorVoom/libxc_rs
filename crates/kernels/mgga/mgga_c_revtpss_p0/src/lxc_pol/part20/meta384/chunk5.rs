//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1407/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1407<F: Float>(t11061: F, t11064: F, t10489: F, t198: F, t207: F, t2403: F, t2404: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39738: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t40975: F, t41023: F, t41075: F, t41131: F, t775: F, t892: F) -> F {
    let t41137 = t11061 * t11064;
    let t41141 = F::new(12.0) * t2403 * t2404 * t10489 - t39528 + t39531 + t39534 + t39537 - t39540 + t198 * t207 * (t40975 + t41023 + t41075 + t41131) * t892 + F::new(24.0) * t2403 * t41137 * t775 + t39738 + t39741 + t39744 + t39747 + t39750 + t39756;
    t41141
}
