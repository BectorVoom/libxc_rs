//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3245/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3245(t2327: f64, t4245: f64, t10194: f64, t10260: f64, t10263: f64, t10415: f64, t1310: f64, t13435: f64, t13514: f64, t13544: f64, t18163: f64, t1843: f64, t2320: f64, t2322: f64, t2328: f64, t2371: f64, t3821: f64, t4248: f64, t4293: f64, t508: f64, t5517: f64, t5787: f64, t651: f64) -> (f64, f64) {
    let t60206 = t4245 * t2327;
    let t60213 = -6.0_f64 * t1310 * t13514 * t651 - 6.0_f64 * t2371 * t5517 * t651 - 6.0_f64 * t10194 * t1843 - 2.0_f64 * t10260 * t4248 - 6.0_f64 * t10263 * t4248 - t10415 * t1843 - 12.0_f64 * t13435 * t4293 - 6.0_f64 * t13544 * t2322 - 6.0_f64 * t18163 * t4293 - 3.0_f64 * t2320 * t5517 - 6.0_f64 * t2328 * t5517 + 3.0_f64 * t3821 * t5787 - 6.0_f64 * t508 * t60206;
    (t60206, t60213)
}
