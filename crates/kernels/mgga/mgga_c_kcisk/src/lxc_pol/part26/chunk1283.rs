//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1283/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1283<F: Float>(t114596: F, t33605: F, t3748: F, t1299: F, t2270: F, t394: F, t5885: F, t3805: F, t9821: F, t1333: F, t33512: F, t33492: F, t33584: F, t33524: F, t9442: F, t21499: F, t33372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t114597 = 0.33163888888888888888e-2 * t114596;
    let t114606 = t3748 * t33605;
    let t114608 = t2270 * t1299;
    let t114618 = t5885 * t394;
    let t114625 = t3805 * t9821;
    let t114628 = t1333 * t33512;
    let t114633 = t1333 * t33492;
    let t114634 = 0.22109259259259259258e-2 * t114633;
    let t114635 = t1333 * t33584;
    let t114636 = 0.33163888888888888888e-2 * t114635;
    let t114638 = 0.69444444444444444446e-2 * t33524 * t9442;
    let t114664 = t33372 * t21499;
    (t114597, t114606, t114608, t114618, t114625, t114628, t114633, t114634, t114635, t114636, t114638, t114664)
}
