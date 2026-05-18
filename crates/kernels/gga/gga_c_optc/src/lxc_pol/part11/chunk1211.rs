//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1211/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1211<F: Float>(t43: F, t13007: F, t16231: F, t1933: F, t22014: F, t3331: F, t4565: F, t55901: F, t55906: F, t55912: F, t607: F, t4570: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t55916 = piecewise3::<f64>(t44, F::new(0.0), -F::new(56.0) / F::new(81.0) * t22014 * t55901 + F::new(16.0) / F::new(9.0) * t13007 * t4565 - F::new(2.0) / F::new(3.0) * t1933 * t55906 - F::new(8.0) / F::new(9.0) * t3331 * t16231 + F::new(2.0) / F::new(3.0) * t607 * t55912);
    let t55917 = t4570 * t4570;
    (t55916, t55917)
}
