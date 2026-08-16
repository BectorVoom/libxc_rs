//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 998/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk998(t15472: f64, t2843: f64, t840: f64, t1212: f64, t2682: f64, t10683: f64, t319: f64, t14603: f64, t296: f64, t1248: f64, t2862: f64, t871: f64) -> (f64, f64, f64, f64) {
    let t15474 = t840 * t2843 * t15472;
    let t15477 = t1212 * t2682;
    let t15479 = t10683 * t319 * t15477;
    let t15482 = t296 * t14603;
    let t15485 = t1248 * t2682;
    let t15487 = t2862 * t871 * t15485;
    (t15474, t15479, t15482, t15487)
}
