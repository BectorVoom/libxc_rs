//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 784/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk784<F: Float>(t28152: F, t787: F, t9824: F, t12656: F, t2684: F, t7354: F, t22629: F, t825: F, t9438: F, t900: F, t9624: F, t10023: F, t10032: F, t2021: F, t7372: F, t2673: F, t40848: F) -> (F, F, F, F, F, F, F, F) {
    let t41468 = t787 * t28152;
    let t41469 = t41468 * t9824;
    let t41474 = t2684 * t7354 * t12656;
    let t41477 = t825 * t9438 * t22629;
    let t41511 = t900 * t9624;
    let t41512 = t10023 * t41511;
    let t41515 = t2021 * t10032 * t7372;
    let t41518 = t2673 * t900 * t40848;
    (t41468, t41469, t41474, t41477, t41511, t41512, t41515, t41518)
}
