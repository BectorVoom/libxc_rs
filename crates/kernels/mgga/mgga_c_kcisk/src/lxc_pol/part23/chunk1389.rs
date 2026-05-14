//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1389/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1389<F: Float>(t1333: F, t33512: F, t2236: F, t3908: F, t415: F, t33492: F, t33584: F, t33524: F, t9442: F, t109883: F, t18989: F, t3482: F, t1327: F, t20053: F, t220: F, t33357: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114628 = t1333 * t33512;
    let t114631 = t415 * t3908 * t2236;
    let t114633 = t1333 * t33492;
    let t114634 = 0.22109259259259259258e-2 * t114633;
    let t114635 = t1333 * t33584;
    let t114636 = 0.33163888888888888888e-2 * t114635;
    let t114638 = 0.69444444444444444446e-2 * t33524 * t9442;
    let t114643 = t3482 * t109883 * t18989;
    let t114651 = t20053 * t33357 * t220 * t1327;
    (t114628, t114631, t114633, t114634, t114635, t114636, t114638, t114643, t114651)
}
