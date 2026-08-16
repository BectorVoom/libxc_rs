//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1054/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1054(t116385: f64, t116387: f64, t117347: f64, t117349: f64, t123337: f64, t124587: f64, t124591: f64, t124596: f64, t1404: f64, t1858: f64, t2105: f64, t27241: f64, t3: f64, t32282: f64, t34077: f64, t5381: f64, t580: f64, t8812: f64) -> f64 {
    let t124600 = t124587 * t3 * t580 + t1404 * t34077 + t1858 * t32282 + 2.0_f64 * t2105 * t27241 + t5381 * t8812 + t116385 + t116387 + 2.0_f64 * t117347 + 2.0_f64 * t117349 + t123337 + t124591 + 2.0_f64 * t124596;
    t124600
}
