//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1090/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1090<F: Float>(t16963: F, t1901: F, t2221: F, t4454: F, t4462: F, t50781: F, t64231: F, t64255: F, t64279: F, t77644: F, t77678: F, t77719: F, t77721: F, t77752: F, t9115: F) -> F {
    let t87754 = -F::new(8.0) / F::new(9.0) * t77644 + F::new(112.0) / F::new(243.0) * t50781 + F::new(4.0) / F::new(3.0) * t77678 + F::new(2.0) / F::new(3.0) * t1901 * t2221 * t16963 * t4462 + F::new(4.0) / F::new(9.0) * t1901 * t9115 * t16963 * t4454 + F::new(8.0) / F::new(9.0) * t64231 + F::new(8.0) / F::new(27.0) * t77719 + F::new(4.0) / F::new(9.0) * t77721 - F::new(4.0) / F::new(9.0) * t77752 - F::new(16.0) / F::new(9.0) * t64255 + F::new(16.0) / F::new(9.0) * t64279;
    t87754
}
