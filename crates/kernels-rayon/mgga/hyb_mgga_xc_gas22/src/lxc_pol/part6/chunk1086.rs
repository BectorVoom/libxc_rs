//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1086/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1086(t10534: f64, t10549: f64, t10567: f64, t10569: f64, t10572: f64, t10578: f64, t10585: f64, t10587: f64, t6530: f64, t6597: f64, t8676: f64, t8830: f64) -> f64 {
    let t10589 = 0.142419375e1_f64 * t10567 - 0.1898925e1_f64 * t10569 - 0.9494625e0_f64 * t10572 + 0.1898925e1_f64 * t10578 - t6597 + 0.39862222222222222223e0_f64 * t6530 + 0.79724444444444444445e0_f64 * t8676 - t8830 - 0.29896666666666666667e0_f64 * t10534 + 0.8969e0_f64 * t10549 - 0.76790625e-1_f64 * t10585 + 0.3071625e0_f64 * t10587;
    t10589
}
