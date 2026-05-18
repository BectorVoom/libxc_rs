//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1092/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1092<F: Float>(t1882: F, t34750: F, t34714: F, t34562: F, t38659: F, t32419: F, t46565: F, t138361: F, t138367: F, t144813: F, t145922: F, t1901: F, t26390: F, t32077: F, t3238: F, t32571: F, t34632: F, t34677: F, t34768: F, t379: F, t446: F, t452: F, t46874: F, t5710: F, t83: F, t8411: F, t8466: F, t8506: F, t8557: F, t986: F) -> (F, F, F, F, F) {
    let t146923 = t1882 * t34750;
    let t146929 = t1882 * t34714;
    let t146937 = t38659 * t34562;
    let t146972 = t46565 * t32419;
    let t146976 = F::new(2.0) / F::new(3.0) * t1901 * t46874 * t144813 - t1901 * t8557 * t34768 * t379 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t8506 * t34677 - F::new(2.0) * t446 * t8411 * t986 * t32077 + t138361 - F::new(2.0) / F::new(9.0) * t138367 - F::new(2.0) / F::new(3.0) * t446 * t452 * t8466 * t34632 - t446 * t83 * t145922 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t452 * t3238 * t32571 + F::new(2.0) / F::new(3.0) * t446 * t452 * t5710 * t26390 - F::new(2.0) * t446 * t83 * t146972;
    (t146923, t146929, t146937, t146972, t146976)
}
