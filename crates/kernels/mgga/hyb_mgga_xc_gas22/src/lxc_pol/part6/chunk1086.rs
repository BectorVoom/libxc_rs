//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1086/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1086<F: Float>(t10534: F, t10549: F, t10567: F, t10569: F, t10572: F, t10578: F, t10585: F, t10587: F, t6530: F, t6597: F, t8676: F, t8830: F) -> F {
    let t10589 = F::new(0.142419375e1) * t10567 - F::new(0.1898925e1) * t10569 - F::new(0.9494625e0) * t10572 + F::new(0.1898925e1) * t10578 - t6597 + F::new(0.39862222222222222223e0) * t6530 + F::new(0.79724444444444444445e0) * t8676 - t8830 - F::new(0.29896666666666666667e0) * t10534 + F::new(0.8969e0) * t10549 - F::new(0.76790625e-1) * t10585 + F::new(0.3071625e0) * t10587;
    t10589
}
