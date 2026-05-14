//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1338/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1338<F: Float>(t13917: F, t34802: F, t9446: F, t26512: F, t3759: F, t9461: F, t1339: F, t32203: F, t8176: F, t110222: F, t114664: F, t114674: F, t119182: F, t119446: F, t2718: F, t32008: F, t32022: F, t32087: F, t33373: F, t33377: F, t33428: F, t33439: F, t33520: F, t34763: F, t34803: F, t6221: F, t9426: F) -> (F, F, F) {
    let t119487 = t9446 * t13917 * t34802;
    let t119494 = t3759 * t9461 * t26512;
    let t119497 = t1339 * t32203 * t8176;
    let t119499 = 0.80416666666666666669e-2 * t9426 * t119182 - 0.20833333333333333334e-1 * t6221 * t33520 * t2718 + 0.69444444444444444447e-2 * t114664 * t33428 + 0.26805555555555555557e-2 * t114674 * t33428 + 0.26805555555555555556e-2 * t110222 * t34763 + 0.41666666666666666668e-1 * t32087 * t119446 + 0.24125000000000000001e-1 * t32008 * t119446 + 0.12345679012345679013e-1 * t32022 * t34803 - 0.15432098765432098766e-2 * t119487 - 0.20833333333333333334e-1 * t33373 * t33439 - 0.80416666666666666669e-2 * t33377 * t33439 + 0.27636574074074074073e-2 * t119494 + 0.22109259259259259258e-2 * t119497;
    (t119494, t119497, t119499)
}
