//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2706/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2706(t1307: f64, t193: f64, t39518: f64, t39521: f64, t39529: f64, t39539: f64, t54420: f64, t54421: f64, t54422: f64, t54423: f64, t54424: f64, t54425: f64, t54427: f64) -> (f64, f64) {
    let t55224 = t193 * t1307;
    let t55228 = t54420 + t54421 - t54422 + t39518 - t39521 - t54423 - t39529 - t54424 - t54425 + t39539 - t54427;
    (t55224, t55228)
}
