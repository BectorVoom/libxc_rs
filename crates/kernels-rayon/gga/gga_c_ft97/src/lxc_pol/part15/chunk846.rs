//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 846/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk846(t10658: f64, t14949: f64, t19838: f64, t19839: f64, t19852: f64, t19857: f64, t19858: f64, t19859: f64, t21981: f64, t22164: f64, t22332: f64, t22336: f64, t22339: f64) -> f64 {
    let t22438 = -2.0_f64 * t21981 - t14949 + t19838 - t19839 - t22164 / 3.0_f64 + t22332 / 6.0_f64 + t22336 / 8.0_f64 - t22339 / 4.0_f64 - t19852 - t10658 + t19857 - t19858 + t19859;
    t22438
}
