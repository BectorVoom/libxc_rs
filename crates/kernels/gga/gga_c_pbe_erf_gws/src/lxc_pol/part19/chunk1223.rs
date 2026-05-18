//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1223/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1223<F: Float>(t2242: F, t4094: F, t51977: F, t4113: F, t1208: F, t6729: F, t51414: F, t51458: F, t4116: F, t6854: F, t13808: F, t14754: F) -> (F, F, F, F, F, F, F, F) {
    let t52560 = t2242 * t4094;
    let t52582 = F::new(455.0) / F::new(648.0) * t51977;
    let t52586 = t2242 * t4113;
    let t52589 = F::new(455.0) / F::new(1296.0) * t6729 * t1208;
    let t52696 = F::new(595.0) / F::new(2592.0) * t51414;
    let t52715 = F::new(455.0) / F::new(648.0) * t51458;
    let t52751 = t4116 * t6854;
    let t52901 = t13808 * t14754;
    (t52560, t52582, t52586, t52589, t52696, t52715, t52751, t52901)
}
