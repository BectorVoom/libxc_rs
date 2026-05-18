//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 918/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk918<F: Float>(t4821: F, t4827: F, t4830: F, t4851: F, t4826: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4856: F, t4864: F, t8031: F, t8033: F, t8034: F, t8035: F) -> (F, F, F, F, F) {
    let t10266 = F::new(8.0) * t4821;
    let t10267 = F::new(32.0) * t4827;
    let t10268 = F::new(20.0) * t4830;
    let t10269 = F::new(0.10843580882781524214e-1) * t4851;
    let t10270 = -t10266 + t4826 + t10267 + t10268 - t8031 - t4837 - t4840 - t4843 + t4846 + t4849 + t10269 + t8033 + t4856 + t8034 - t8035 - t4864;
    (t10266, t10267, t10268, t10269, t10270)
}
