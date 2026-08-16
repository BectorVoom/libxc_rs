//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1066/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1066(t4778: f64, t2086: f64, t91: f64, t20758: f64, t2992: f64, t1969: f64, t446: f64, t49266: f64, t62246: f64, t77914: f64, t77917: f64, t77920: f64, t77935: f64, t77990: f64, t86986: f64, t86989: f64, t86992: f64, t86995: f64, t86998: f64, t87002: f64) -> (f64, f64, f64, f64) {
    let t87004 = t4778 * t4778;
    let t87006 = t91 * t2086 * t87004;
    let t87009 = t2992 * t20758;
    let t87011 = t446 * t1969 * t87009;
    let t87016 = 8.0_f64 / 3.0_f64 * t77914 + 8.0_f64 / 9.0_f64 * t77917 + 40.0_f64 / 243.0_f64 * t77920 - t86986 / 3.0_f64 + 8.0_f64 / 9.0_f64 * t86989 - 8.0_f64 / 27.0_f64 * t86992 + 4.0_f64 / 9.0_f64 * t86995 - 4.0_f64 * t86998 + 2.0_f64 * t87002 - t87006 / 4.0_f64 + 4.0_f64 / 9.0_f64 * t77935 - 8.0_f64 / 3.0_f64 * t87011 - 8.0_f64 / 9.0_f64 * t62246 + 112.0_f64 / 81.0_f64 * t49266 - 8.0_f64 / 9.0_f64 * t77990;
    (t87006, t87009, t87011, t87016)
}
