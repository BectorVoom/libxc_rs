//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1002/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1002<F: Float>(t144: F, t167: F, t1901: F, t2075: F, t2179: F, t2180: F, t2185: F, t2205: F, t2210: F, t3440: F, t379: F, t38064: F, t38930: F, t39658: F, t40840: F, t40847: F, t446: F, t558: F, t569: F, t574: F, t616: F, t7959: F, t9007: F, t9276: F, t9311: F, t9327: F, t9344: F, t9419: F, t9462: F) -> F {
    let t40880 = F::new(8.0) / F::new(9.0) * t40840 + F::new(8.0) / F::new(3.0) * t446 * t2185 * t167 * t9007 * t558 - F::new(8.0) / F::new(9.0) * t40847 - F::new(12.0) * t446 * t144 * t39658 - F::new(4.0) * t446 * t574 * t2179 * t2180 * t2075 - F::new(8.0) * t446 * t574 * t9276 * t9311 + F::new(16.0) / F::new(9.0) * t446 * t2205 * t616 * t7959 + F::new(40.0) / F::new(27.0) * t446 * t9327 * t167 * t38064 - F::new(4.0) / F::new(9.0) * t446 * t569 * t9462 * t379 - F::new(4.0) * t1901 * t2210 * t3440 * t38930 - F::new(8.0) / F::new(3.0) * t1901 * t9419 * t9344;
    t40880
}
