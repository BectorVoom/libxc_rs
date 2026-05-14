//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1097/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1097<F: Float>(t37431: F, t37438: F, t40303: F, t40305: F, t40308: F, t40313: F, t40315: F, t40320: F, t40331: F, t40334: F, t42862: F, t42866: F, t42871: F, t42874: F, t43854: F, t37443: F, t37444: F, t37448: F, t37452: F, t40342: F, t40346: F, t42876: F, t42881: F, t42885: F, t42889: F, t42893: F, t42897: F, t42900: F, t42904: F, t42908: F) -> (F, F) {
    let t43864 = 0.15243824895787514157e-3 * t43854 - 0.38422568777328955684e-2 * t40303 + 0.92232789896410962678e-3 * t40305 + 0.72042316457491791906e-3 * t40308 + t40313 - 0.86737941314158990623e-4 * t40315 - t40320 + t42862 - 0.72042316457491791906e-3 * t37431 + 0.10248087766267884742e-3 * t37438 + t42866 + 0.16260079888840015101e-2 * t40331 - 0.3903207359137154578e-3 * t40334 - t42871 + t42874;
    let t43867 = t37443 - t42876 + t40342 - t40346 + t42881 - t42885 - t42889 + t42893 - t42897 + t42900 + 0.30487649791575028314e-3 * t37444 - t37448 + t42904 - t37452 - t42908;
    (t43864, t43867)
}
