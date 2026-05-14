//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1116/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1116<F: Float>(t35890: F, t35895: F, t35898: F, t35901: F, t35903: F, t35907: F, t35909: F, t35912: F, t35915: F, t35919: F, t35921: F, t35923: F, t35925: F, t11629: F, t3254: F, t10102: F, t3724: F) -> (F, F, F) {
    let t35927 = -0.34197428278281706076e-6 * t35890 - 0.3077768545045353547e-5 * t35895 - 0.4892908831675294957e-7 * t35898 + 0.64219428415738246312e-6 * t35901 - 0.23485962392041415794e-3 * t35903 - 0.64219428415738246312e-6 * t35907 + 0.23485962392041415794e-4 * t35909 - 0.64219428415738246312e-6 * t35912 + 0.59785630648647397395e-7 * t35915 + 0.73393632475129424356e-6 * t35919 - 0.16146599144528473358e-4 * t35921 - 0.93943849568165663176e-4 * t35923 - 0.10149523886505120173e-5 * t35925;
    let t35928 = t3254 * t11629;
    let t35930 = t10102 * t3724;
    (t35927, t35928, t35930)
}
