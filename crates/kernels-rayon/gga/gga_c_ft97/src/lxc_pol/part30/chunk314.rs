//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 314/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk314(t2681: f64, t4218: f64, t824: f64, t192: f64, t4129: f64, t852: f64, t2761: f64, t2762: f64, t2764: f64, t3139: f64, t4197: f64, t4200: f64, t4203: f64, t4207: f64, t4210: f64, t4213: f64, t4215: f64, t462: f64, t92: f64) -> f64 {
    let t4220 = t2681 * t4218 * t824;
    let t4224 = t192 * t852 * t4129;
    let t4226 = t2761 + t2762 / 9.0_f64 + t2764 / 3.0_f64 + t4197 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t462 * t4200 + t462 * t4203 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t462 * t4207 + 2.0_f64 / 3.0_f64 * t3139 * t4210 + t4213 / 3.0_f64 + t462 * t4215 / 3.0_f64 + 2.0_f64 * t462 * t4220 - t92 * t4224;
    t4226
}
