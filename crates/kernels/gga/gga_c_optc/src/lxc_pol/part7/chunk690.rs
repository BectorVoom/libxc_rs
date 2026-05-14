//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 690/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk690<F: Float>(t6359: F, t6437: F, t6625: F, t6627: F, t6634: F, t6638: F, t6640: F, t6644: F, t6647: F, t6694: F, t6696: F, t6709: F, t6737: F, t2042: F, t592: F, t6449: F, t6457: F, t6465: F, t6477: F, t6741: F, t6744: F, t6747: F, t6750: F, t6753: F, t6771: F, t6773: F) -> (F, F, F) {
    let t6808 = t6625 - t6627 - t6634 - t6638 - t6640 - t6644 - t6647 - t6694 - t6696 - t6709 + t6359 + t6737 - t6437;
    let t6811 = 60.0 * t2042 * t592;
    let t6812 = t6449 + t6457 + t6741 + t6744 - t6747 - t6750 + t6753 + t6465 + t6771 + t6773 + t6811 + t6477;
    (t6808, t6811, t6812)
}
