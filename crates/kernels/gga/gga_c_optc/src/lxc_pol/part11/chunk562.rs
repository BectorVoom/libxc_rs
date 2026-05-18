//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 562/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk562<F: Float>(t43: F, t50: F, t40: F, t4580: F, t1933: F, t4561: F, t4565: F, t607: F, t1940: F, t4570: F, t4573: F, t611: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t4581 = t40 * t4580;
    let t4587 = piecewise3::<f64>(t44, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1933 * t4561 + F::new(2.0) / F::new(3.0) * t607 * t4565);
    let t4593 = piecewise3::<f64>(t51, F::new(0.0), -F::new(2.0) / F::new(9.0) * t1940 * t4570 + F::new(2.0) / F::new(3.0) * t611 * t4573);
    let t4595 = t4587 / F::new(2.0) + t4593 / F::new(2.0);
    (t4581, t4595)
}
