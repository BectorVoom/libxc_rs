//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1212/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1212(t1218: f64, t21930: f64, t22469: f64, t312: f64, t317: f64, t5304: f64, t5305: f64, t5422: f64, t788: f64, t90775: f64, t90785: f64, t90803: f64, t90873: f64, t90936: f64, t91005: f64, t91082: f64, t91125: f64, t91158: f64, t91171: f64, t91186: f64, t91195: f64) -> f64 {
    let t91216 = -t788 * (t91158 + t91171 + t91186 + t91195) * t317 + 12.0_f64 * t91125 + 48.0_f64 * t90936 - 72.0_f64 * t90873 - 12.0_f64 * t90775 + 16.0_f64 * t90785 - 3.0_f64 * t21930 * t5304 * t317 + 2.0_f64 * t91082 * t312 + 48.0_f64 * t91005 - 48.0_f64 * t90803 - 6.0_f64 * t5305 * t5422 - 4.0_f64 * t1218 * t22469;
    t91216
}
