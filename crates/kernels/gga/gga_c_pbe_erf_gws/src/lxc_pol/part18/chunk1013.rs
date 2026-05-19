//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1013/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1013<F: Float>(t6080: F, t10263: F, t10264: F, t10266: F, t10267: F, t10268: F, t10269: F, t4826: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4856: F, t4864: F, t8031: F, t8033: F, t8034: F, t8035: F) -> F {
    let t11313 = F::cast_from(0.6846054806677777778e0_f64) * t6080;
    let t11314 = t10263 - t10264 - t10266 + t4826 + t10267 + t10268 - t8031 - t4837 - t4840 - t4843 + t4846 + t4849 + t10269 + t8033 + t11313 + t4856 + t8034 - t8035 - t4864;
    t11314
}
