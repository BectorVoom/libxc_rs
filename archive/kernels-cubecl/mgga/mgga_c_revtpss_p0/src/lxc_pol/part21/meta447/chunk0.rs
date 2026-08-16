//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1972/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1972<F: Float>(t13610: F, t13638: F, t13663: F, t14308: F, t1532: F, t2609: F, t10437: F, t2398: F, t4308: F, t4305: F, t262: F, t4343: F) -> (F, F, F, F, F, F) {
    let t14310 = t13610 + t13638 + t13663 + t14308;
    let t14312 = t1532 * t2609;
    let t14313 = F::cast_from(4.0_f64) * t10437;
    let t14315 = F::cast_from(8.0_f64) * t2398 * t4308;
    let t14317 = F::cast_from(8.0_f64) * t2398 * t4305;
    let t14318 = t262 * t4343;
    (t14310, t14312, t14313, t14315, t14317, t14318)
}
