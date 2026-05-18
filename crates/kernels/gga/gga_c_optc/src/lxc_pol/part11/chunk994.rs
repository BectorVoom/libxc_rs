//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 994/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk994<F: Float>(t106: F, t1147: F, t12522: F, t1550: F, t15706: F, t17947: F, t17960: F, t17964: F, t18174: F, t4403: F, t4410: F, t470: F, t5351: F, t5430: F, t8997: F) -> F {
    let t18178 = F::new(0.27818116767324025134e1) * t106 * t17947 * t470 - F::new(0.83454350301972075402e1) * t106 * t15706 * t1550 + F::new(0.16690870060394415081e2) * t106 * t12522 * t5351 - F::new(0.83454350301972075402e1) * t106 * t4403 * t5430 - F::new(0.1669087006039441508e2) * t106 * t8997 * t17960 + F::new(0.16690870060394415081e2) * t4410 * t17964 - F::new(0.27818116767324025134e1) * t106 * t1147 * t18174;
    t18178
}
