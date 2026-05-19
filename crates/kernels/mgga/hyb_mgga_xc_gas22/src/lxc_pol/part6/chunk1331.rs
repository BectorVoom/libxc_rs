//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1331/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1331<F: Float>(t28877: F, t28880: F, t28883: F, t28885: F, t28887: F, t28890: F, t28892: F, t28894: F, t28896: F, t28899: F, t28901: F, t28904: F) -> F {
    let t29010 = F::cast_from(0.1151859375e0_f64) * t28877 - F::new(0.76790625e-1) * t28880 - F::cast_from(0.3560484375e1_f64) * t28883 + F::cast_from(0.142419375e1_f64) * t28885 - F::new(0.1898925e1) * t28887 - F::new(0.1898925e1) * t28890 - F::new(0.9494625e0) * t28892 - F::new(0.76790625e-1) * t28894 + F::new(0.3071625e0) * t28896 + F::new(0.3071625e0) * t28899 + F::new(0.15358125e0) * t28901 - F::new(0.3071625e0) * t28904;
    t29010
}
