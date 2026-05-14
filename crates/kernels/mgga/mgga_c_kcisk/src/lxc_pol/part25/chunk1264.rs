//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1264/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1264<F: Float>(t1333: F, t32966: F, t33187: F, t4419: F, t9725: F, t18682: F, t25: F, t33228: F, t33196: F, t33207: F, t9724: F, t33179: F, t2804: F, t33192: F, t12261: F, t9747: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t112739 = t1333 * t32966;
    let t112750 = t4419 * t33187;
    let t112751 = t9725 * t112750;
    let t112761 = t25 * t18682;
    let t112762 = t112761 * t33228;
    let t112763 = t33196 * t112762;
    let t112765 = t9724 * t33207;
    let t112772 = t4419 * t33179;
    let t112773 = t2804 * t112772;
    let t112776 = t2804 * t4419 * t33192;
    let t112780 = t2804 * t12261 * t9747;
    (t112739, t112750, t112751, t112761, t112762, t112763, t112765, t112772, t112773, t112776, t112780)
}
