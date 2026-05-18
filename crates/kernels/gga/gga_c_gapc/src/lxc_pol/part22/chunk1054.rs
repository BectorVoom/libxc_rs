//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1054/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1054<F: Float>(t11659: F, t11664: F, t11666: F, t11671: F, t11676: F, t11680: F, t11685: F, t11689: F, t11692: F, t11696: F, t11699: F, t11701: F, t11703: F) -> F {
    let t12190 = -F::new(0.4892908831675294957e-7) * t11659 + F::new(0.64219428415738246311e-6) * t11664 - F::new(0.23485962392041415794e-4) * t11666 - F::new(0.23485962392041415794e-4) * t11671 + F::new(0.34197428278281706076e-6) * t11676 + F::new(0.34197428278281706076e-6) * t11680 - F::new(0.14678726495025884871e-5) * t11685 - F::new(0.14678726495025884871e-5) * t11689 - F::new(0.41758041133049637282e-5) * t11692 + F::new(0.11399142759427235359e-6) * t11696 - F::new(0.54715885245250729723e-5) * t11699 + F::new(0.16414765573575218917e-4) * t11701 - F::new(0.7113065081882594864e-4) * t11703;
    t12190
}
