//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 866/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk866<F: Float>(t10746: F, t184: F, t1024: F, t7951: F, t10711: F, t10714: F, t10717: F, t10721: F, t10726: F, t10728: F, t10730: F, t10732: F, t10734: F, t10736: F, t10738: F, t10739: F, t10741: F, t10745: F, t7578: F) -> (F, F, F) {
    let t10747 = t10746 * t184;
    let t10749 = 8.0 / 15.0 * t10747 * t1024;
    let t10751 = 8.0 / 15.0 * t7951 * t1024;
    let t10752 = -t10711 - t10714 + t10717 + t10721 + t10726 + t10728 - t10730 - t10732 + t10734 + t10736 + t10738 + t7578 - t10739 - t10741 - t10745 + t10749 + t10751;
    (t10749, t10751, t10752)
}
