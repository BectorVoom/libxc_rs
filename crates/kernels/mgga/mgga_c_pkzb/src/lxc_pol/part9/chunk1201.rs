//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1201/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1201<F: Float>(t20759: F, t20762: F, t20765: F, t20769: F, t20773: F, t20777: F, t20781: F, t20789: F, t20791: F, t20794: F, t20797: F, t20800: F, t20803: F, t20806: F, t20809: F, t20811: F, t20813: F, t20815: F, t20817: F) -> F {
    let t20888 = -F::cast_from(0.49293999999999999999e0_f64) * t20759 - F::cast_from(0.98587999999999999999e0_f64) * t20762 - F::cast_from(0.49293999999999999999e0_f64) * t20765 + F::new(0.24647e0) * t20769 + F::new(0.73941e0) * t20773 + F::new(0.73941e0) * t20777 + F::new(0.24647e0) * t20781 + F::new(0.3071625e0) * t20789 + F::new(0.1898925e1) * t20791 + F::cast_from(0.427258125e1_f64) * t20794 - F::cast_from(0.230371875e0_f64) * t20797 - F::cast_from(0.3560484375e1_f64) * t20800 + F::cast_from(0.1151859375e0_f64) * t20803 + F::new(0.46074375e0) * t20806 - F::new(0.28483875e1) * t20809 - F::new(0.28483875e1) * t20811 - F::new(0.9494625e0) * t20813 + F::new(0.46074375e0) * t20815 + F::new(0.15358125e0) * t20817;
    t20888
}
