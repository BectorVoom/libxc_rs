//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 740/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk740<F: Float>(t10000: F, t10004: F, t10009: F, t10012: F, t10015: F, t10020: F, t1901: F, t193: F, t446: F, t89: F, t9845: F, t9850: F, t9855: F, t9976: F, t9982: F, t9985: F, t9989: F, t9993: F, t9997: F) -> F {
    let t10022 = F::new(2.0) * t446 * t9845 + t1901 * t9850 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t1901 * t9855 + t89 * t193 * t9976 / F::new(3.0) - t9982 + t1901 * t9985 / F::new(3.0) - t446 * t9989 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t446 * t9993 - t9997 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t10000 + F::new(2.0) * t446 * t10004 - F::new(2.0) / F::new(3.0) * t1901 * t10009 - F::new(2.0) / F::new(9.0) * t10012 - F::new(2.0) / F::new(9.0) * t1901 * t10015 + t446 * t10020;
    t10022
}
