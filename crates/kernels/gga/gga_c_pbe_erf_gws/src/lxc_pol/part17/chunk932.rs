//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 932/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk932<F: Float>(t127: F, t1533: F, t2893: F, t481: F, t5784: F, t5788: F, t5806: F, t8155: F, t8158: F, t8160: F, t8162: F, t8171: F, t8174: F, t8177: F, t8202: F) -> F {
    let t8204 = t8155 + t8158 - F::new(0.48968e0) * t8160 + F::new(0.1175232e2) * t127 * t8162 * t481 + F::new(0.587616e1) * t127 * t2893 * t1533 + t8171 + t8174 - F::new(4.0) / F::new(9.0) * t5784 + t5788 / F::new(6.0) + t8177 - F::new(0.293808e1) * t5806 + t8202;
    t8204
}
