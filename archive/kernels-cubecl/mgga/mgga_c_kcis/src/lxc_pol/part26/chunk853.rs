//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 853/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk853<F: Float>(t1532: F, t1929: F, t2060: F, t577: F, t1467: F, t12520: F, t492: F, t16751: F, t3393: F, t5989: F, t531: F, t5867: F) -> (F, F, F, F, F, F, F) {
    let t17474 = t1532 * t1929;
    let t17504 = t577 * t2060;
    let t17505 = t1467 * t17504;
    let t17508 = t12520 * t492;
    let t17514 = t16751 * t577;
    let t17540 = t3393 * t5989;
    let t17546 = t5867 * t531;
    (t17474, t17504, t17505, t17508, t17514, t17540, t17546)
}
