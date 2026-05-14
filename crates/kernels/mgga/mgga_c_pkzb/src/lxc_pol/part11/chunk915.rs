//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 915/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk915<F: Float>(t12: F, t10495: F, t10537: F, t10747: F, t10751: F, t45: F, t2865: F, t3605: F, t730: F, t10513: F, t5528: F, t10518: F, t652: F, t2732: F, t3366: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t10753 = t10495 + t10537 + t10747 + t10751;
    let t10754 = t45 * t10753;
    let t10755 = t2865 * t3605;
    let t10757 = 0.35089341735807877242e1 * t730 * t10755;
    let t10760 = t5528 * t10513;
    let t10764 = t652 * t10518;
    let t10767 = piecewise3(t84, 0.0, -28.0 / 27.0 * t10760 + 4.0 / 3.0 * t2732 * t3366 - t10764 / 3.0);
    (t10753, t10754, t10755, t10757, t10760, t10764, t10767)
}
