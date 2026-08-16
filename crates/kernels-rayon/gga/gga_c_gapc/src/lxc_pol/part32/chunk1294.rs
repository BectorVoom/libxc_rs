//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1294/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1294(t2152: f64, t2208: f64, t3649: f64, t3739: f64, t10226: f64, t11640: f64, t828: f64, t10230: f64, t11633: f64, t35890: f64, t35895: f64, t35898: f64, t35901: f64, t35903: f64, t35907: f64, t35909: f64, t35912: f64, t35915: f64) -> f64 {
    let t35919 = t3649 * t2152 * t2208 * t3739;
    let t35921 = t10226 * t3739;
    let t35923 = t828 * t11640;
    let t35925 = t10230 * t11633;
    let t35927 = -0.34197428278281706076e-6_f64 * t35890 - 0.3077768545045353547e-5_f64 * t35895 - 0.4892908831675294957e-7_f64 * t35898 + 0.64219428415738246312e-6_f64 * t35901 - 0.23485962392041415794e-3_f64 * t35903 - 0.64219428415738246312e-6_f64 * t35907 + 0.23485962392041415794e-4_f64 * t35909 - 0.64219428415738246312e-6_f64 * t35912 + 0.59785630648647397395e-7_f64 * t35915 + 0.73393632475129424356e-6_f64 * t35919 - 0.16146599144528473358e-4_f64 * t35921 - 0.93943849568165663176e-4_f64 * t35923 - 0.10149523886505120173e-5_f64 * t35925;
    t35927
}
