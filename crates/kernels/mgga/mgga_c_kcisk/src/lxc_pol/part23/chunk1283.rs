//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1283/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1283<F: Float>(t32215: F, t3973: F, t9446: F, t21499: F, t32101: F, t110324: F, t9426: F, t13900: F, t9448: F, t3969: F, t13485: F, t32087: F, t32089: F, t32065: F, t110577: F, t13917: F, t32130: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110615 = t9446 * t3973 * t32215;
    let t110635 = t32101 * t21499;
    let t110640 = t9426 * t110324;
    let t110648 = t9446 * t13900 * t9448;
    let t110655 = t32101 * t3969;
    let t110661 = t32087 * t13485 * t32089;
    let t110663 = t32065 * t21499;
    let t110666 = t32087 * t110577;
    let t110673 = t9446 * t13917 * t32130;
    (t110615, t110635, t110640, t110648, t110655, t110661, t110663, t110666, t110673)
}
