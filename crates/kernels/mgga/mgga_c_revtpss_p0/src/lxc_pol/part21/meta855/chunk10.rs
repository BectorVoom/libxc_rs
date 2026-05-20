//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3245/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3245<F: Float>(t2327: F, t4245: F, t10194: F, t10260: F, t10263: F, t10415: F, t1310: F, t13435: F, t13514: F, t13544: F, t18163: F, t1843: F, t2320: F, t2322: F, t2328: F, t2371: F, t3821: F, t4248: F, t4293: F, t508: F, t5517: F, t5787: F, t651: F) -> (F, F) {
    let t60206 = t4245 * t2327;
    let t60213 = -F::new(6.0) * t1310 * t13514 * t651 - F::new(6.0) * t2371 * t5517 * t651 - F::new(6.0) * t10194 * t1843 - F::new(2.0) * t10260 * t4248 - F::new(6.0) * t10263 * t4248 - t10415 * t1843 - F::new(12.0) * t13435 * t4293 - F::new(6.0) * t13544 * t2322 - F::new(6.0) * t18163 * t4293 - F::new(3.0) * t2320 * t5517 - F::new(6.0) * t2328 * t5517 + F::new(3.0) * t3821 * t5787 - F::new(6.0) * t508 * t60206;
    (t60206, t60213)
}
