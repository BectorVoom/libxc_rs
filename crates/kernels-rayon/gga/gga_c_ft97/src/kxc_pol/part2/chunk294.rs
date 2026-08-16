//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 294/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk294(t1240: f64, t295: f64, t312: f64, t1188: f64, t1215: f64, t1236: f64, t873: f64) -> (f64, f64) {
    let t1242 = t295 * t1240 * t312;
    let t1248 = t1236 / 2.0_f64 - t873 - t1188 / 3.0_f64 - t1215;
    (t1242, t1248)
}
