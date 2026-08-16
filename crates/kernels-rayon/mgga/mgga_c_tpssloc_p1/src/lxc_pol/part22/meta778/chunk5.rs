//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2668/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2668(t54412: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t39490: f64, t39496: f64, t54401: f64, t54403: f64, t54409: f64, t74056: f64, t74057: f64, t74073: f64, t74075: f64, t74078: f64, t74086: f64) -> (f64, f64) {
    let t74470 = 36.0_f64 * t54412;
    let t74471 = -t74056 + t39463 - t39468 + t74057 + t54401 - t39472 - t39476 - t54403 + t74073 - t74075 - t74078 + t54409 + t74086 + t39483 - t74470 - t39490 - t39496;
    (t74470, t74471)
}
