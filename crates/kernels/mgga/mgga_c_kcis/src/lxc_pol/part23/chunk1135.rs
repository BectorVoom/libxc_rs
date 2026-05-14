//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1135/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1135<F: Float>(t27369: F, t98847: F, t16618: F, t303: F, t553: F, t12231: F, t6140: F, t1014: F, t28525: F, t16761: F, t28524: F, t3955: F, t1394: F, t5644: F, t94216: F, t27484: F, t8151: F) -> (F, F, F, F, F, F, F, F, F) {
    let t98854 = 0.61836467013888888889e-4 * t27369 * t98847;
    let t98856 = t303 * t553 * t16618;
    let t98860 = t12231 * t6140;
    let t98863 = t1014 * t28525;
    let t98864 = 0.33163888888888888888e-2 * t98863;
    let t98866 = t303 * t553 * t16761;
    let t98869 = t303 * t28524 * t3955;
    let t98872 = t1394 * t94216 * t5644;
    let t98874 = t8151 * t27484;
    (t98854, t98856, t98860, t98863, t98864, t98866, t98869, t98872, t98874)
}
