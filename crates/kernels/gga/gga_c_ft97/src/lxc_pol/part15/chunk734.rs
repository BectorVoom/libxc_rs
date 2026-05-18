//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 734/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk734<F: Float>(t20031: F, t3499: F, t20556: F, t2102: F, t20560: F, t20336: F, t582: F, t17279: F, t17281: F, t20786: F, t20789: F, t20793: F, t20796: F, t462: F, t9178: F) -> (F, F, F, F, F) {
    let t20799 = t3499 * t20031;
    let t20802 = t2102 * t20556;
    let t20804 = t2102 * t20560;
    let t20806 = t582 * t20336;
    let t20809 = -F::new(2.0) * t462 * t20786 - F::new(2.0) * t462 * t20789 - t9178 + t17279 - F::new(2.0) * t17281 + F::new(2.0) / F::new(3.0) * t462 * t20793 + F::new(4.0) / F::new(3.0) * t462 * t20796 - F::new(2.0) / F::new(3.0) * t462 * t20799 + t462 * t20802 + t462 * t20804 - t462 * t20806 / F::new(3.0);
    (t20799, t20802, t20804, t20806, t20809)
}
