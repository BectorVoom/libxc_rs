//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 873/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk873<F: Float>(t41965: F, t6717: F, t6914: F, t10532: F, t10533: F, t42188: F, t42190: F, t42194: F, t42198: F, t42200: F, t42203: F, t42205: F, t42208: F, t42210: F, t42214: F, t42216: F, t42221: F, t42224: F, t42227: F, t42230: F, t42233: F, t42236: F, t42239: F) -> F {
    let t42242 = F::cast_from(0.62115540045351614476e2_f64) * t6914 * t6717 * t41965;
    let t42245 = F::cast_from(0.27606906686822939767e2_f64) * t10532 * t10533 * t41965;
    let t42246 = -t42188 + t42190 - t42194 + t42198 - t42200 - t42203 - t42205 - t42208 - F::cast_from(0.21450293971110256002e1_f64) * t42210 - F::cast_from(0.21450293971110256002e1_f64) * t42214 - F::cast_from(0.21450293971110256002e1_f64) * t42216 - t42221 - t42224 - t42227 - t42230 + t42233 + t42236 + t42239 - t42242 + t42245;
    t42246
}
