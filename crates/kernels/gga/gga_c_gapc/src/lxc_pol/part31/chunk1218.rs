//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1218/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1218<F: Float>(t33935: F, t36761: F, t36762: F, t36763: F, t36765: F, t36766: F, t36767: F, t36768: F, t36769: F, t36770: F, t36771: F, t33969: F, t36773: F, t36774: F, t36775: F, t36777: F, t36778: F, t36779: F, t36780: F, t36781: F, t36782: F, t36783: F) -> (F, F) {
    let t38777 = t36761 - t36762 + t36763 + 0.25301106770833333334e-5 * t33935 + t36765 + t36766 + t36767 + t36768 + t36769 - t36770 - t36771;
    let t38779 = -t36773 - t36774 + t36775 - 0.25301106770833333334e-5 * t33969 + t36777 - t36778 + t36779 + t36780 - t36781 - t36782 - t36783;
    (t38777, t38779)
}
