//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 781/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk781<F: Float>(t1413: F, t1449: F, t2481: F, t2507: F, t3311: F, t3337: F, t4218: F, t430: F, t453: F, t459: F, t4772: F, t4828: F, t6634: F, t8599: F, t8604: F, t8607: F, t8610: F, t8615: F, t8661: F, t8664: F, t8667: F, t8670: F, t8673: F, t8676: F, t8705: F) -> (F,) {
    let t8708 = 0.33125e-1 * t4218 * t2507 - 0.33125e-1 * t8599 * t459 - 0.6625e-1 * t6634 * t3311 + 0.19875e0 * t4772 * t8604 - 0.6625e-1 * t1413 * t8607 - 0.6625e-1 * t1413 * t8610 + 0.165625e-1 * t2481 * t3337 - 0.33125e-1 * t1413 * t8615 + 0.165625e-1 * t430 * t8661 + 0.99375e-1 * t4772 * t8664 - 0.19875e0 * t4828 * t8667 + 0.99375e-1 * t1449 * t8670 - 0.33125e-1 * t1413 * t8673 + 0.496875e-1 * t1449 * t8676 - 0.165625e-1 * t453 * t8705;
    (t8708,)
}
