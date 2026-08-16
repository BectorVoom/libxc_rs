//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1248/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1248(t33153: f64, t10627: f64, t1835: f64, t7572: f64, t7573: f64, t10914: f64, t10915: f64, t32897: f64, t25198: f64, t7391: f64, t3487: f64, t739: f64, t7803: f64, t7805: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33154 = 0.51762950037793012063e1_f64 * t33153;
    let t33155 = t10627 * t1835;
    let t33158 = 0.69017266717057349418e1_f64 * t7572 * t7573 * t33155;
    let t33164 = 0.42900587942220512002e1_f64 * t10914 * t10915 * t32897;
    let t33178 = t25198 * t7391;
    let t33179 = 0.89376224879626066674e-1_f64 * t33178;
    let t33182 = t7803 * t739 * t3487 * t7805;
    (t33154, t33155, t33158, t33164, t33179, t33182)
}
