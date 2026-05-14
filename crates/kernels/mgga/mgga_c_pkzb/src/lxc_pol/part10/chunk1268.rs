//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1268/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1268<F: Float>(t16569: F, t16575: F, t16578: F, t24531: F, t24532: F, t24533: F, t24535: F, t24537: F, t24540: F, t24543: F, t24544: F, t24546: F, t16592: F, t16599: F, t24598: F, t24601: F, t24602: F, t24605: F, t24607: F, t24608: F, t24609: F, t24610: F, t2536: F, t2537: F, t7177: F) -> (F, F) {
    let t24952 = t16569 - t24531 + t16575 + t16578 + t24532 + t24533 + t24535 - t24537 - t24540 - t24543 - t24544 + t24546;
    let t24957 = -2.0 * t2536 * t2537 * t7177 - t16592 + t16599 + t24598 + t24601 + t24602 + t24605 + t24607 - t24608 - t24609 - t24610;
    (t24952, t24957)
}
