//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1066/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1066<F: Float>(t4778: F, t2086: F, t91: F, t20758: F, t2992: F, t1969: F, t446: F, t49266: F, t62246: F, t77914: F, t77917: F, t77920: F, t77935: F, t77990: F, t86986: F, t86989: F, t86992: F, t86995: F, t86998: F, t87002: F) -> (F, F, F, F) {
    let t87004 = t4778 * t4778;
    let t87006 = t91 * t2086 * t87004;
    let t87009 = t2992 * t20758;
    let t87011 = t446 * t1969 * t87009;
    let t87016 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t77914 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t77917 + F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t77920 - t86986 / F::cast_from(3.0_f64) + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t86989 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t86992 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t86995 - F::cast_from(4.0_f64) * t86998 + F::cast_from(2.0_f64) * t87002 - t87006 / F::cast_from(4.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t77935 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t87011 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t62246 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t49266 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t77990;
    (t87006, t87009, t87011, t87016)
}
