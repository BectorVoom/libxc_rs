//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1286/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1286(t16288: f64, t6417: f64, t12385: f64, t20497: f64, t20433: f64, t3866: f64, t16336: f64, t6431: f64, t1831: f64, t57021: f64, t53945: f64, t6396: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74217 = t16288 * t6417;
    let t74228 = t12385 * t20497;
    let t74256 = t3866 * t20433;
    let t74258 = t16336 * t6431;
    let t74260 = t57021 * t1831;
    let t74274 = t53945 * t6396;
    (t74217, t74228, t74256, t74258, t74260, t74274)
}
