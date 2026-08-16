//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 428/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk428(t2360: f64, t312: f64, t1242: f64, t681: f64, t89: f64, t1225: f64, t1882: f64, t1221: f64, t1258: f64, t5: f64, t1263: f64, t2253: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4265 = t312 * t2360;
    let t4271 = t89 * t681 * t1242;
    let t4273 = t1882 * t1225;
    let t4283 = t1882 * t1221;
    let t4322 = t5 * t1258;
    let t4332 = t2253 * t1263;
    (t4265, t4271, t4273, t4283, t4322, t4332)
}
