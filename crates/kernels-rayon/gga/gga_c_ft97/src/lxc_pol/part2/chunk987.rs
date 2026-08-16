//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 987/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk987(t10514: f64, t15256: f64, t15260: f64, t15263: f64, t15267: f64, t15271: f64, t15273: f64, t15274: f64, t15277: f64, t15281: f64, t15286: f64, t15291: f64, t15296: f64, t15300: f64, t15304: f64, t1901: f64, t3281: f64, t446: f64) -> f64 {
    let t15307 = -4.0_f64 / 9.0_f64 * t1901 * t15256 - 2.0_f64 / 9.0_f64 * t1901 * t15260 + 2.0_f64 / 3.0_f64 * t446 * t15263 + 2.0_f64 / 9.0_f64 * t3281 * t15267 + t15271 + t15273 - t446 * t15274 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t15277 + 2.0_f64 / 3.0_f64 * t446 * t15281 + 2.0_f64 / 3.0_f64 * t446 * t15286 + 8.0_f64 / 27.0_f64 * t10514 + 4.0_f64 / 27.0_f64 * t1901 * t15291 + 4.0_f64 / 27.0_f64 * t1901 * t15296 - 4.0_f64 / 9.0_f64 * t1901 * t15300 - 2.0_f64 / 9.0_f64 * t1901 * t15304;
    t15307
}
