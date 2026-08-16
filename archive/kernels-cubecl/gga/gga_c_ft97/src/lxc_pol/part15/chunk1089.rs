//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1089/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1089<F: Float>(t1053: F, t77633: F, t144: F, t167: F, t1901: F, t20868: F, t2185: F, t2210: F, t2221: F, t3440: F, t446: F, t49622: F, t50679: F, t64001: F, t76618: F, t77491: F, t77505: F, t77521: F, t86681: F, t87091: F, t87295: F, t925: F) -> (F, F) {
    let t87699 = t77633 * t1053;
    let t87707 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t77491 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t77505 + F::cast_from(2.0_f64) * t446 * t2185 * t167 * t86681 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t64001 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t77521 - F::cast_from(2.0_f64) * t446 * t144 * t87091 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t2221 * t3440 * t87295 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t2210 * t76618 * t925 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t87699 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t50679 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t49622 * t20868;
    (t87699, t87707)
}
