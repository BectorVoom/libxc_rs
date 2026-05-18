//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 857/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk857<F: Float>(t17338: F, t4822: F, t558: F, t12791: F, t17334: F, t11755: F, t11761: F, t12852: F, t12864: F, t12865: F, t17296: F, t17299: F, t17302: F, t17305: F, t17310: F, t17313: F, t17316: F, t17319: F, t17322: F, t17325: F, t17328: F, t17331: F, t17335: F, t3139: F, t462: F, t9178: F, t9202: F) -> F {
    let t17340 = t17338 * t4822 * t558;
    let t17343 = t12791 * t17334;
    let t17346 = -F::new(10.0) / F::new(27.0) * t462 * t17296 - F::new(8.0) / F::new(9.0) * t3139 * t17299 + F::new(2.0) / F::new(3.0) * t462 * t17302 + t462 * t17305 / F::new(3.0) - F::new(8.0) / F::new(27.0) * t12852 - t12864 + F::new(4.0) / F::new(9.0) * t12865 - t9178 - F::new(2.0) / F::new(9.0) * t17310 - F::new(4.0) / F::new(27.0) * t9202 - F::new(2.0) / F::new(3.0) * t462 * t17313 + t462 * t17316 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t462 * t17319 - F::new(2.0) / F::new(9.0) * t462 * t17322 - F::new(2.0) / F::new(3.0) * t462 * t17325 - F::new(2.0) * t462 * t17328 + F::new(8.0) / F::new(3.0) * t3139 * t17331 + F::new(4.0) / F::new(9.0) * t11755 * t17335 - F::new(4.0) / F::new(3.0) * t11761 * t17340 - F::new(4.0) / F::new(3.0) * t11761 * t17343;
    t17346
}
