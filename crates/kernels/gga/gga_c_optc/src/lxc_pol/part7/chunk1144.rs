//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1144/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1144<F: Float>(t2391: F, t2399: F, t2382: F, t214: F, t2383: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23653: F, t23655: F, t23670: F, t23673: F, t23676: F, t23679: F) -> (F, F, F, F, F) {
    let t23839 = t2391 * t2391;
    let t23840 = t2399 * t23839;
    let t23842 = t2382 * t23839;
    let t23844 = f64::powf(t214, -F::new(0.25e1));
    let t23845 = t2383 * t2383;
    let t23846 = t23844 * t23845;
    let t23859 = -F::new(8.0) * t23605 + F::new(8.0) * t23670 - F::new(2.0) / F::new(3.0) * t23608 - F::new(8.0) / F::new(9.0) * t23673 - F::new(20.0) / F::new(9.0) * t23676 + F::new(8.0) * t23612 - F::new(12.0) * t23679 + F::new(16.0) / F::new(9.0) * t23614 + F::new(8.0) / F::new(3.0) * t23616 - F::new(8.0) / F::new(3.0) * t23653 + F::new(8.0) / F::new(9.0) * t23655;
    (t23840, t23842, t23845, t23846, t23859)
}
