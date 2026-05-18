//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1074/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1074<F: Float>(t240: F, t9577: F, t1526: F, t9483: F, t9499: F, t15567: F, t17687: F, t17694: F, t2320: F, t3806: F, t42264: F, t42267: F, t42270: F, t42273: F, t9490: F, t9514: F, t9571: F, t9583: F, t9592: F, t9775: F) -> F {
    let t42279 = t240 * t9577;
    let t42288 = t1526 * t9483 * t9499;
    let t42290 = -t1526 * t2320 * t9490 * t9571 / F::new(2.0) + t15567 * t17694 * t9592 / F::new(2.0) + t42264 / F::new(18.0) - t42267 / F::new(6.0) - t42270 / F::new(12.0) - t42273 / F::new(9.0) + F::new(2.0) * t9514 + t1526 * t2320 * t9775 / F::new(2.0) + F::new(2.0) / F::new(3.0) * t1526 * t3806 * t42279 * t9571 - t15567 * t17687 * t9583 / F::new(3.0) + t42288 / F::new(6.0);
    t42290
}
