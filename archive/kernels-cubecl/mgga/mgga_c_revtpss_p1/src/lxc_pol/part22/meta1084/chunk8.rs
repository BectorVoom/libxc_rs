//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3933/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3933<F: Float>(t13426: F, t13514: F, t13517: F, t13537: F, t1519: F, t18163: F, t18227: F, t1843: F, t1911: F, t21882: F, t21891: F, t3813: F, t3821: F, t4248: F, t4254: F, t4257: F, t4293: F, t49686: F, t508: F, t5887: F, t5920: F, t651: F, t6934: F, t75485: F, t75494: F, t75657: F, t75667: F, t94: F) -> F {
    let t75714 = -F::cast_from(4.0_f64) * t13514 * t1843 * t651 - F::cast_from(2.0_f64) * t3813 * t5920 * t651 - F::cast_from(2.0_f64) * t508 * t651 * t75657 - F::cast_from(4.0_f64) * t508 * t75494 * t94 - F::cast_from(8.0_f64) * t13426 * t4257 - F::cast_from(8.0_f64) * t13426 * t4293 + F::cast_from(2.0_f64) * t13517 * t1911 - F::cast_from(4.0_f64) * t13537 * t4248 - F::cast_from(4.0_f64) * t1519 * t49686 - F::cast_from(4.0_f64) * t1519 * t75485 - F::cast_from(8.0_f64) * t1519 * t75667 - F::cast_from(4.0_f64) * t18163 * t5887 - F::cast_from(8.0_f64) * t18227 * t4257 - F::cast_from(8.0_f64) * t18227 * t4293 - F::cast_from(4.0_f64) * t21882 * t4254 - F::cast_from(8.0_f64) * t21891 * t4254 + t3821 * t6934;
    t75714
}
