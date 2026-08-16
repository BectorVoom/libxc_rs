//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 965/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk965(t10613: f64, t14653: f64, t1775: f64, t4215: f64, t14660: f64, t2771: f64, t14889: f64, t192: f64, t852: f64, t302: f64, t668: f64, t683: f64) -> (f64, f64, f64, f64, f64) {
    let t14995 = t10613 * t14653;
    let t14999 = 2.0_f64 / 9.0_f64 * t1775 * t4215;
    let t15000 = t2771 * t14660;
    let t15004 = t192 * t852 * t14889;
    let t15007 = t683 * t302 * t668;
    (t14995, t14999, t15000, t15004, t15007)
}
