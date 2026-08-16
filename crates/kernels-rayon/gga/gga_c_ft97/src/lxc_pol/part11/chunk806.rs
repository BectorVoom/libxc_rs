//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 806/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk806(t2440: f64, t70: f64, t327: f64, t9570: f64, t9571: f64, t8640: f64, t895: f64, t2253: f64, t2934: f64, t2920: f64, t2941: f64, t10871: f64, t10875: f64, t10877: f64, t10896: f64, t10900: f64, t10907: f64, t10912: f64, t2265: f64, t631: f64) -> (f64, f64, f64, f64) {
    let t10915 = t70 * t2440;
    let t10916 = t327 * t9570;
    let t10918 = t10915 * t10916 * t9571;
    let t10921 = t8640 * t895;
    let t10923 = t2253 * t2934;
    let t10925 = t2253 * t2920;
    let t10927 = t2253 * t2941;
    let t10929 = 2.0_f64 * t2265 * t10871 + t631 * t10875 - t2265 * t10877 + t631 * t10896 / 2.0_f64 + t631 * t10900 / 6.0_f64 + 6.0_f64 * t631 * t10907 - 9.0_f64 / 2.0_f64 * t631 * t10912 + 2.0_f64 / 27.0_f64 * t631 * t10918 + 5.0_f64 / 9.0_f64 * t10921 - t10923 / 3.0_f64 - t10925 / 9.0_f64 + 3.0_f64 * t10927;
    (t10915, t10916, t10918, t10929)
}
