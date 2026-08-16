//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 939/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk939(t22817: f64, t7203: f64, t1669: f64, t15: f64, t32163: f64, t5555: f64, t32161: f64, t22755: f64, t9: f64, t420: f64, t5578: f64, t1608: f64, t32167: f64, t32237: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t136433 = t22817 * t7203;
    let t136434 = t1669 * t136433;
    let t136457 = t5555 * t15 * t32163;
    let t136458 = t32161 * t136457;
    let t136468 = t1669 * t22755 * t9;
    let t136469 = t5578 * t420;
    let t136474 = t1608 * t32167 * t32237;
    (t136433, t136434, t136457, t136458, t136468, t136469, t136474)
}
