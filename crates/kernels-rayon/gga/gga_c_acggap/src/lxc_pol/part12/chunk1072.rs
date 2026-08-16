//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1072/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1072(t137: f64, t336: f64, t4876: f64, t578: f64, t2068: f64, t4680: f64, t8911: f64, t1181: f64, t23688: f64, t599: f64, t7346: f64, t7433: f64, t8966: f64) -> (f64, f64, f64, f64) {
    let t35080 = t578 * t336 * t4876 * t137;
    let t35084 = t2068 * t4680 * t8911;
    let t35088 = t7346 * t1181 * t599 * t23688;
    let t35090 = t7433 * t8966;
    (t35080, t35084, t35088, t35090)
}
