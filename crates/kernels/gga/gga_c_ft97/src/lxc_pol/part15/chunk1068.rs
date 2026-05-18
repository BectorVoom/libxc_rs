//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1068/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1068<F: Float>(t2205: F, t446: F, t85491: F, t86622: F, t9049: F, t1969: F, t86669: F, t62287: F, t62309: F, t62317: F, t78001: F, t78012: F, t78015: F, t78027: F, t87024: F, t87027: F, t87030: F, t87033: F, t87037: F) -> (F, F, F, F) {
    let t87042 = t446 * t2205 * t85491;
    let t87045 = t446 * t9049 * t86622;
    let t87048 = t446 * t1969 * t86669;
    let t87050 = -F::new(8.0) / F::new(27.0) * t78001 + F::new(4.0) / F::new(27.0) * t78012 - F::new(16.0) / F::new(27.0) * t78015 - F::new(8.0) / F::new(3.0) * t78027 - F::new(8.0) / F::new(27.0) * t62287 - F::new(4.0) / F::new(3.0) * t87024 - F::new(4.0) / F::new(3.0) * t87027 - F::new(8.0) / F::new(9.0) * t87030 - F::new(16.0) / F::new(9.0) * t87033 - t87037 / F::new(3.0) - F::new(16.0) / F::new(81.0) * t62309 + F::new(16.0) / F::new(27.0) * t62317 + F::new(8.0) / F::new(3.0) * t87042 + F::new(4.0) / F::new(9.0) * t87045 + F::new(8.0) / F::new(3.0) * t87048;
    (t87042, t87045, t87048, t87050)
}
