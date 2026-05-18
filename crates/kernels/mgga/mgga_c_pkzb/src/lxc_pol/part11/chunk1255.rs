//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1255/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1255<F: Float>(t3605: F, t730: F, t7527: F, t25671: F, t2852: F, t3618: F, t7560: F, t7299: F, t9351: F, t20982: F, t9531: F, t2865: F, t9465: F) -> (F, F, F, F, F, F) {
    let t30764 = F::new(0.35089341735807877242e1) * t730 * t7527 * t3605;
    let t30767 = F::new(0.51947577317044391277e2) * t730 * t25671 * t2852;
    let t30769 = F::new(0.35089341735807877242e1) * t7560 * t3618;
    let t30772 = F::new(0.51947577317044391277e2) * t730 * t9351 * t7299;
    let t30775 = F::new(0.30762056574649219974e4) * t730 * t9531 * t20982;
    let t30778 = F::new(0.35089341735807877242e1) * t730 * t2865 * t9465;
    (t30764, t30767, t30769, t30772, t30775, t30778)
}
