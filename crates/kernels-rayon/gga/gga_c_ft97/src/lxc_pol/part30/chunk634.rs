//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 634/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk634(t1456: f64, t2574: f64, t3837: f64, t3746: f64, t724: f64, t1091: f64, t6194: f64, t242: f64, t28100: f64, t1901: f64, t24605: f64, t24611: f64, t28195: f64, t28198: f64, t28201: f64, t28205: f64, t28209: f64, t28212: f64, t28214: f64, t3281: f64, t446: f64) -> f64 {
    let t28218 = t2574 * t1456 * t3837;
    let t28222 = t724 * t1456 * t3746;
    let t28226 = t724 * t6194 * t1091;
    let t28230 = t242 * t28100;
    let t28233 = -t446 * t28195 / 3.0_f64 - t446 * t28198 / 3.0_f64 - t446 * t28201 / 3.0_f64 - t1901 * t28205 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t28209 + t28212 / 9.0_f64 + t28214 / 9.0_f64 - t24605 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t28218 + 2.0_f64 / 9.0_f64 * t3281 * t28222 - t446 * t28226 / 9.0_f64 - t24611 / 9.0_f64 - t446 * t28230 / 3.0_f64;
    t28233
}
