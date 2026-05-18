//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1022/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1022<F: Float>(t11690: F, t1787: F, t3127: F, t3134: F, t38464: F, t38478: F, t38483: F, t44950: F, t462: F, t8291: F, t85456: F, t85465: F, t85474: F, t85483: F, t86054: F, t86058: F, t86068: F, t86075: F, t86082: F, t86086: F, t86090: F, t86094: F, t86098: F) -> F {
    let t86102 = F::new(112.0) / F::new(27.0) * t44950 + F::new(4.0) / F::new(3.0) * t462 * t1787 * t86054 + F::new(4.0) / F::new(3.0) * t462 * t1787 * t86058 + F::new(8.0) / F::new(3.0) * t462 * t3134 * t85456 - F::new(8.0) / F::new(9.0) * t462 * t3127 * t85465 + F::new(40.0) / F::new(27.0) * t462 * t38483 * t86068 - F::new(20.0) / F::new(9.0) * t462 * t11690 * t85483 + F::new(8.0) * t462 * t1787 * t86075 - F::new(12.0) * t462 * t3134 * t85474 + F::new(2.0) * t462 * t1787 * t86082 - F::new(4.0) * t462 * t8291 * t86086 + F::new(8.0) * t462 * t38478 * t86090 + F::new(8.0) * t462 * t8291 * t86094 - F::new(8.0) / F::new(3.0) * t462 * t38464 * t86098;
    t86102
}
