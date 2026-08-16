//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 437/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk437(t28: f64, t5890: f64, t6657: f64, t1969: f64, t5900: f64, t925: f64, t5899: f64, t2112: f64, t6630: f64, t1369: f64, t586: f64, t6615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6659 = t5890 * t28 * t6657;
    let t6662 = t1969 * t5900 * t925;
    let t6663 = t5899 * t6662;
    let t6665 = t2112 * t6630;
    let t6667 = t1369 * t28 * t6665;
    let t6669 = t586 * t6615;
    (t6659, t6662, t6663, t6665, t6667, t6669)
}
