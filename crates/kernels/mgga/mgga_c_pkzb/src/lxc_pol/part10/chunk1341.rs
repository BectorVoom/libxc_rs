//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1341/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1341<F: Float>(t2993: F, t135: F, t2156: F, t25619: F, t25622: F, t25626: F, t25644: F, t25647: F, t25651: F, t25653: F, t25655: F, t25658: F, t25660: F, t25662: F, t25664: F, t25666: F, t273: F) -> (F,) {
    let t26785 = t2993 * t2993;
    let t26790 = -2.0 * t135 * t2156 * t26785 * t273 - t25619 + t25622 + t25626 + t25644 + t25647 + t25651 - t25653 - t25655 - t25658 - t25660 + t25662 + t25664 + t25666;
    (t26790,)
}
