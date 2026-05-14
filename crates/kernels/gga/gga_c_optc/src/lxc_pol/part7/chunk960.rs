//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 960/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk960<F: Float>(t22497: F, t22562: F, t22578: F, t22581: F, t22593: F, t22681: F, t22683: F, t22685: F, t22687: F, t22690: F, t22694: F, t22697: F, t6742: F, t6751: F, t1796: F, t509: F, t6636: F) -> (F, F, F) {
    let t22698 = t22681 - t22683 - t22685 + t22687 - t22690 - t22694 - t22497 + t22562 + t22578 + t22581 - t22593 + t22697;
    let t22699 = t6742 * t6751;
    let t22700 = 0.1926377843805564792e1 * t22699;
    let t22703 = 0.38024868119570572865e2 * t1796 * t509 * t6636;
    (t22698, t22700, t22703)
}
