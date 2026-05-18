//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1154/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1154<F: Float>(t10690: F, t1882: F, t2832: F, t848: F, t10810: F, t1901: F, t2682: F, t2862: F, t2884: F, t2894: F, t296: F, t319: F, t43525: F, t44272: F, t44276: F, t44278: F, t44280: F, t44289: F, t44292: F, t44294: F, t446: F, t684: F, t835: F) -> F {
    let t44300 = t1882 * t10690;
    let t44302 = t848 * t2832;
    let t44306 = -F::new(2.0) * t446 * t296 * t44272 + F::new(4.0) / F::new(3.0) * t44276 + F::new(4.0) / F::new(9.0) * t44278 + F::new(8.0) * t446 * t44280 * t319 * t43525 - F::new(4.0) / F::new(9.0) * t446 * t835 * t10810 * t684 - F::new(8.0) / F::new(3.0) * t44289 + F::new(8.0) / F::new(9.0) * t44292 - F::new(16.0) / F::new(9.0) * t44294 + F::new(4.0) * t446 * t2862 * t2894 * t2682 - F::new(8.0) / F::new(3.0) * t44300 + F::new(4.0) / F::new(3.0) * t1901 * t44302 * t2884;
    t44306
}
