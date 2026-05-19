//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 731/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk731<F: Float>(t4541: F, t1: F, t2057: F, t793: F, t2062: F, t1423: F, t414: F, t700: F, t762: F, t1354: F, t145: F, t242: F) -> (F, F, F, F, F, F) {
    let t4542 = F::new(6.0) * t4541;
    let t4544 = t793 * t2057 * t1;
    let t4545 = t4544 * t2062;
    let t4547 = t414 * t1423;
    let t4550 = F::cast_from(0.50257692321302641125e0_f64) * t762 * t700;
    let t4551 = t145 * t1354;
    let t4552 = t4551 * t242;
    (t4542, t4545, t4547, t4550, t4551, t4552)
}
