//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1294/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1294<F: Float>(t2152: F, t2208: F, t3649: F, t3739: F, t10226: F, t11640: F, t828: F, t10230: F, t11633: F, t35890: F, t35895: F, t35898: F, t35901: F, t35903: F, t35907: F, t35909: F, t35912: F, t35915: F) -> F {
    let t35919 = t3649 * t2152 * t2208 * t3739;
    let t35921 = t10226 * t3739;
    let t35923 = t828 * t11640;
    let t35925 = t10230 * t11633;
    let t35927 = -F::new(0.34197428278281706076e-6) * t35890 - F::new(0.3077768545045353547e-5) * t35895 - F::new(0.4892908831675294957e-7) * t35898 + F::new(0.64219428415738246312e-6) * t35901 - F::new(0.23485962392041415794e-3) * t35903 - F::new(0.64219428415738246312e-6) * t35907 + F::new(0.23485962392041415794e-4) * t35909 - F::new(0.64219428415738246312e-6) * t35912 + F::new(0.59785630648647397395e-7) * t35915 + F::new(0.73393632475129424356e-6) * t35919 - F::new(0.16146599144528473358e-4) * t35921 - F::new(0.93943849568165663176e-4) * t35923 - F::new(0.10149523886505120173e-5) * t35925;
    t35927
}
