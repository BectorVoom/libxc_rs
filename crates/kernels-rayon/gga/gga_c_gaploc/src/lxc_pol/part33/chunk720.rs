//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 720/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk720(t1457: f64, t6321: f64, t4752: f64, t494: f64, t1344: f64, t1645: f64, t188: f64, t6316: f64, t1340: f64, t2345: f64, t4673: f64, t493: f64, t6519: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6734 = t1457 * t6321;
    let t6737 = t4752 * t494;
    let t6740 = t1645 * t1344;
    let t6743 = t188 * t6316;
    let t6744 = t1645 * t1340;
    let t6747 = t4673 * t2345;
    let t6750 = t493 * t6519;
    (t6734, t6737, t6740, t6743, t6744, t6747, t6750)
}
