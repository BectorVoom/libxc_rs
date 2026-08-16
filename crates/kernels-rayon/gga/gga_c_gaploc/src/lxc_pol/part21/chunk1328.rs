//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1328/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1328(t10513: f64, t20800: f64, t4391: f64, t10525: f64, t1564: f64, t539: f64, t32033: f64, t6716: f64, t6717: f64, t10533: f64, t20796: f64, t1397: f64, t8237: f64, t9287: f64) -> (f64, f64, f64, f64, f64) {
    let t34668 = 0.57200783922960682671e1_f64 * t4391 * t20800 * t10513;
    let t34672 = 0.28600391961480341335e1_f64 * t10525 * t539 * t1564 * t10513;
    let t34675 = 0.12423108009070322895e3_f64 * t6716 * t6717 * t32033;
    let t34678 = 0.55213813373645879534e2_f64 * t20796 * t10533 * t32033;
    let t34680 = t1397 * t8237 * t9287;
    (t34668, t34672, t34675, t34678, t34680)
}
