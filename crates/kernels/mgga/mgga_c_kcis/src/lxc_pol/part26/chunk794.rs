//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 794/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk794<F: Float>(t1467: F, t17504: F, t12520: F, t492: F, t16751: F, t577: F, t3393: F, t5989: F, t531: F, t5867: F, t14955: F, t5977: F, t5969: F, t11670: F, t538: F, t2018: F, t456: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17505 = t1467 * t17504;
    let t17508 = t12520 * t492;
    let t17514 = t16751 * t577;
    let t17540 = t3393 * t5989;
    let t17546 = t5867 * t531;
    let t17583 = t14955 * t5977;
    let t17586 = 0.5895802469135802469e-1 * t14955 * t5969;
    let t17594 = t11670 * t538;
    let t17613 = t2018 * t456;
    (t17505, t17508, t17514, t17540, t17546, t17583, t17586, t17594, t17613)
}
