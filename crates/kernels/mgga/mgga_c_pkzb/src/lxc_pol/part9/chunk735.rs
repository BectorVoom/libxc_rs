//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 735/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk735<F: Float>(t24: F, t1655: F, t5106: F, t5107: F, t5110: F, t5113: F, t91: F, t5104: F, t98: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t5117 = piecewise3::<f64>(t90, F::new(0.0), -F::new(8.0) / F::new(27.0) * t5106 * t5107 + F::new(4.0) / F::new(3.0) * t5110 * t1655 + F::new(4.0) / F::new(3.0) * t91 * t5113);
    let t5119 = (t5104 + t5117) * t98;
    t5119
}
