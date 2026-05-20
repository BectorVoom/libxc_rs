//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3824/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3824<F: Float>(t6922: F, t9593: F, t22185: F, t2619: F, t48277: F, t47672: F, t6781: F, t13600: F, t13625: F, t13716: F, t13867: F, t13872: F, t22475: F, t4139: F, t4144: F, t47067: F, t5532: F, t5536: F, t5537: F, t5541: F, t5627: F, t6836: F, t9547: F) -> (F, F, F) {
    let t73499 = t6922 * t9593;
    let t73515 = t22185 * t2619;
    let t73516 = F::cast_from(0.24415263074675393405e-3_f64) * t73515;
    let t73517 = F::cast_from(0.36622894612013090108e-3_f64) * t48277;
    let t73518 = t6781 * t47672;
    let t73528 = F::new(24.0) * t13600 * t5536 * t5627 + F::new(12.0) * t13625 * t22475 * t4139 + F::new(12.0) * t13716 * t5536 * t5537 + F::new(24.0) * t13867 * t5532 * t5536 + F::new(12.0) * t13872 * t5532 * t5536 + F::new(2.0) * t4144 * t5541 * t73499 - F::new(6.0) * t4144 * t5541 * t73518 + F::new(6.0) * t5536 * t6836 * t9547 + t47067 + t73516 - t73517;
    (t73516, t73517, t73528)
}
