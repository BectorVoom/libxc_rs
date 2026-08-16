//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1831/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1831(t22844: f64, t6976: f64, t5259: f64, t80820: f64, t80767: f64, t80776: f64, t22779: f64, t26292: f64, t80784: f64, t80792: f64, t80794: f64, t16060: f64, t6944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t91208 = t22844 * t6976;
    let t91214 = t80820 * t5259;
    let t91221 = 0.13565246047631171327e0_f64 * t80767;
    let t91223 = 35.0_f64 / 108.0_f64 * t80776;
    let t91225 = t22779 * t26292;
    let t91244 = 0.33643963411783659044e-4_f64 * t80784;
    let t91246 = 0.10541775202358879834e-2_f64 * t80792;
    let t91247 = 119.0_f64 / 3456.0_f64 * t80794;
    let t91278 = t16060 * t6944;
    (t91208, t91214, t91221, t91223, t91225, t91244, t91246, t91247, t91278)
}
