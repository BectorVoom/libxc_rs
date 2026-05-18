//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1247/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1247<F: Float>(t1053: F, t1102: F, t1103: F, t9005: F, t37431: F, t37438: F, t40303: F, t40305: F, t40308: F, t40313: F, t40315: F, t40320: F, t40331: F, t40334: F, t42862: F, t42866: F, t42871: F, t42874: F) -> F {
    let t43854 = t1102 * t1053 * t1103 * t9005;
    let t43864 = F::new(0.15243824895787514157e-3) * t43854 - F::new(0.38422568777328955684e-2) * t40303 + F::new(0.92232789896410962678e-3) * t40305 + F::new(0.72042316457491791906e-3) * t40308 + t40313 - F::new(0.86737941314158990623e-4) * t40315 - t40320 + t42862 - F::new(0.72042316457491791906e-3) * t37431 + F::new(0.10248087766267884742e-3) * t37438 + t42866 + F::new(0.16260079888840015101e-2) * t40331 - F::new(0.3903207359137154578e-3) * t40334 - t42871 + t42874;
    t43864
}
