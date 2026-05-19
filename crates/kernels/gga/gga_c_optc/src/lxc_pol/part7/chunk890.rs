//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 890/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk890<F: Float>(t2993: F, t8565: F, t1055: F, t3012: F, t3020: F, t3018: F, t1036: F, t2985: F, t1057: F, t2988: F, t3013: F, t1032: F, t3017: F) -> (F, F, F, F, F, F, F) {
    let t8567 = F::new(6.0) * t2993 * t8565;
    let t8569 = t3012 * t3020 * t1055;
    let t8571 = F::cast_from(0.48245472966453314466e2_f64) * t3018 * t8569;
    let t8572 = t2985 * t1036;
    let t8574 = F::new(3.0) * t8572 * t1057;
    let t8576 = F::new(3.0) * t2988 * t3013;
    let t8577 = t1032 * t3017;
    (t8567, t8569, t8571, t8572, t8574, t8576, t8577)
}
