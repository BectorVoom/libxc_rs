//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1348/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1348<F: Float>(t118710: F, t118738: F, t118777: F, t118815: F, t118861: F, t118894: F, t118930: F, t118965: F, t119006: F, t119037: F, t119064: F, t119093: F, t119123: F, t119151: F, t119179: F, t119218: F, t119250: F, t119283: F, t119315: F, t119341: F, t119379: F, t119413: F, t119440: F, t119465: F, t119499: F, t119529: F, t119557: F, t119586: F, t119629: F, t119647: F, t119672: F, t119703: F, t504: F) -> (F,) {
    let t119709 = (t119250 + t119006 + t118930 + t118738 + t119179 + t119529 + t119037 + t119123 + t119465 + t118861 + t119413 + t118710 + t118894 + t119499 + t119064 + t119218 + t119379 + t119586 + t119440 + t119151 + t118815 + t118965 + t118777 + t119647 + t119283 + t119672 + t119557 + t119703 + t119315 + t119341 + t119629 + t119093) * t504;
    (t119709,)
}
