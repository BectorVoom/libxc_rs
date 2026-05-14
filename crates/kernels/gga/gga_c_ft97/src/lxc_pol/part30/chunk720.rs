//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 720/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk720<F: Float>(t33966: F, t824: F, t193: F, t89: F, t6222: F, t6260: F, t375: F, t7658: F, t668: F, t7611: F) -> (F, F, F, F, F, F, F) {
    let t33967 = t33966 * t824;
    let t33968 = t193 * t33967;
    let t33969 = t89 * t33968;
    let t33971 = t6222 * t6260;
    let t33972 = t193 * t33971;
    let t33973 = t89 * t33972;
    let t33976 = t89 * t375 * t7658;
    let t33977 = t33976 / 9.0;
    let t33978 = t7611 * t668;
    (t33967, t33969, t33971, t33973, t33976, t33977, t33978)
}
