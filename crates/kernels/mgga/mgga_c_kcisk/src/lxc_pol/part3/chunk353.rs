//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 353/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk353<F: Float>(t645: F, t1751: F, t1758: F, t340: F, t639: F, t642: F, t655: F, t1720: F, t397: F, t662: F, t656: F, t122: F, sigma2: F) -> (F, F, F, F, F) {
    let t646 = t645 < -F::new(0.66725e-1);
    let t1763 = piecewise3::<F>(t646, F::new(0.0), F::new(10.0) / F::new(9.0) * t340 * t1751 * t642 - F::new(10.0) / F::new(27.0) * t340 * t639 * t1758);
    let t1764 = t1763 * sigma2;
    let t1765 = t1764 * t655;
    let t1769 = t397 * t1720 * t662;
    let t1771 = F::cast_from(0.17990788716177317213e-1_f64) * t656 * t1769;
    let t1772 = t655 * t122;
    (t1764, t1765, t1769, t1771, t1772)
}
