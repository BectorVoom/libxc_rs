//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 793/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk793<F: Float>(t21606: F, t21623: F, t734: F, t91: F, t13722: F, t13739: F, t17720: F, t21402: F, t21414: F, t21419: F, t21422: F, t21433: F, t21437: F, t21448: F, t21556: F, t21567: F) -> (F, F, F) {
    let t21624 = t21606 + t21623;
    let t21626 = t91 * t734 * t21624;
    let t21628 = -F::new(2.0) / F::new(3.0) * t17720 - t21402 - F::new(6.0) * t21419 - F::new(3.0) / F::new(4.0) * t21556 - t21414 / F::new(3.0) + F::new(6.0) * t21422 - F::new(10.0) / F::new(27.0) * t21433 - F::new(2.0) * t21437 + F::new(4.0) / F::new(3.0) * t21448 - F::new(4.0) / F::new(9.0) * t13722 - F::new(4.0) / F::new(3.0) * t13739 + F::new(3.0) / F::new(8.0) * t21567 + t21626 / F::new(2.0);
    (t21624, t21626, t21628)
}
