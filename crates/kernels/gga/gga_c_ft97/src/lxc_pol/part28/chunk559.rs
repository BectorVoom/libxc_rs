//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 559/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk559<F: Float>(t23075: F, t1882: F, t5657: F, t1328: F, t1637: F, t89: F, t5724: F, t1314: F, t8232: F, t376: F, t5706: F, t5637: F) -> (F, F, F, F, F, F, F) {
    let t23124 = F::new(4.0) / F::new(27.0) * t23075;
    let t23148 = t1882 * t5657;
    let t23152 = F::new(4.0) / F::new(27.0) * t89 * t1637 * t1328;
    let t23176 = t1882 * t5724;
    let t23183 = F::new(4.0) / F::new(27.0) * t8232 * t1314;
    let t23199 = t89 * t376 * t5706;
    let t23227 = t1882 * t5637;
    (t23124, t23148, t23152, t23176, t23183, t23199, t23227)
}
