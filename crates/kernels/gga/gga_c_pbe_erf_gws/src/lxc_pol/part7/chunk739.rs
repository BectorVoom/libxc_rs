//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 739/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk739<F: Float>(t309: F, t310: F, t311: F, t305: F, t296: F, t413: F, t4652: F, t4664: F, t4747: F, t4751: F, t4754: F, t4756: F, t4780: F, t4784: F, t4786: F, t4790: F, t4792: F, t4795: F, t4797: F) -> (F, F, F, F, F) {
    let t6072 = F::new(1.0) / t311 / t310 / t309;
    let t6073 = t305 * t6072;
    let t6074 = t413 * t296;
    let t6075 = t6073 * t6074;
    let t6076 = F::new(0.47400060215270560269e0) * t6075;
    let t6077 = t4747 + t4751 + t4652 + t4754 + t4756 + t4664 - t6076 + t4780 - t4784 - t4786 - t4790 - t4792 - t4795 + t4797;
    (t6072, t6073, t6074, t6075, t6077)
}
