//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 595/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk595<F: Float>(t4541: F, t1: F, t2057: F, t793: F, t2062: F, t1423: F, t414: F, t700: F, t762: F, t1354: F, t145: F) -> (F, F, F, F, F, F, F, F) {
    let t4542 = F::new(6.0) * t4541;
    let t4544 = t793 * t2057 * t1;
    let t4545 = t4544 * t2062;
    let t4546 = F::new(0.18981728898494541632e1) * t4545;
    let t4547 = t414 * t1423;
    let t4548 = F::new(12.0) * t4547;
    let t4550 = F::new(0.50257692321302641125e0) * t762 * t700;
    let t4551 = t145 * t1354;
    (t4542, t4544, t4545, t4546, t4547, t4548, t4550, t4551)
}
