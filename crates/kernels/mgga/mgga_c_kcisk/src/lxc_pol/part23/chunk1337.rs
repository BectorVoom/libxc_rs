//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1337/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1337<F: Float>(t3508: F, t6336: F, t109294: F, t6328: F, t21038: F, t33652: F, t32295: F, t33643: F, t32269: F, t33658: F, t109217: F, t9836: F, t4193: F, t5606: F, t1299: F, t6387: F) -> (F, F, F, F, F, F, F, F) {
    let t113464 = t3508 * t6336;
    let t113466 = t109294 * t6328;
    let t113468 = t33652 * t21038;
    let t113470 = t33643 * t32295;
    let t113472 = t32269 * t33658;
    let t113474 = t109217 * t9836;
    let t113476 = t5606 * t4193;
    let t113478 = t6387 * t1299;
    (t113464, t113466, t113468, t113470, t113472, t113474, t113476, t113478)
}
