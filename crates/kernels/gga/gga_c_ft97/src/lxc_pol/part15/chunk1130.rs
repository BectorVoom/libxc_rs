//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1130/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1130<F: Float>(t683: F, t88756: F, t92: F, t88149: F, t41446: F, t88252: F, t9568: F, t88612: F, t66202: F, t80096: F, t88737: F, t88740: F, t88744: F, t88747: F, t88751: F, t88754: F) -> (F, F, F, F, F, F) {
    let t88758 = t92 * t683 * t88756;
    let t88761 = t92 * t683 * t88149;
    let t88764 = t41446 * t88252;
    let t88766 = t92 * t9568 * t88764;
    let t88769 = t92 * t9568 * t88612;
    let t88772 = -F::new(8.0) * t88737 + F::new(8.0) * t88740 - F::new(2.0) / F::new(3.0) * t88744 - F::new(8.0) / F::new(9.0) * t88747 + F::new(8.0) * t88751 - F::new(12.0) * t88754 + F::new(2.0) * t88758 + F::new(8.0) / F::new(3.0) * t88761 - F::new(8.0) / F::new(9.0) * t66202 + F::new(40.0) / F::new(9.0) * t88766 - F::new(20.0) / F::new(9.0) * t88769 + F::new(4.0) / F::new(9.0) * t80096;
    (t88758, t88761, t88764, t88766, t88769, t88772)
}
