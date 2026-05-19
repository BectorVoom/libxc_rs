//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 907/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk907<F: Float>(t10051: F, t10054: F, t10090: F, t10154: F, t10158: F, t10162: F, t2911: F, t2912: F, t5753: F, t5755: F, t5776: F, t5863: F, t5864: F, t8137: F, t8142: F, t8231: F) -> F {
    let t10167 = t5753 - t5755 - t10051 + t10054 - F::new(0.2069106e2) * t2911 * t8231 * t10154 + F::new(0.1034553e2) * t2911 * t2912 * t10158 + F::new(0.5172765e1) * t2911 * t2912 * t10162 - t5863 - t5776 - t8137 + t8142 + t10090 - F::cast_from(0.76633555555555555554e0_f64) * t5864;
    t10167
}
