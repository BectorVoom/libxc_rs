//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2772/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2772<F: Float>(t10489: F, t10618: F, t10635: F, t10818: F, t14468: F, t14643: F, t14648: F, t14649: F, t14652: F, t1553: F, t1555: F, t225: F, t227: F, t229: F, t2394: F, t2430: F, t2639: F, t4409: F, t4415: F, t4416: F, t50151: F, t50391: F, t50844: F, t50845: F, t50847: F, t50848: F, t50851: F, t50854: F, t50882: F, t50908: F, t775: F, t832: F, t853: F) -> F {
    let t50914 = F::cast_from(180.0_f64) * t4415 * t14648 * t10818 + F::cast_from(3.0_f64) * t227 * t832 * t50151 + F::cast_from(180.0_f64) * t4415 * t50391 * t2394 - F::cast_from(36.0_f64) * t4415 * t853 * t14468 * t775 - F::cast_from(36.0_f64) * t4415 * t14652 * t2430 + F::cast_from(3.0_f64) * t1553 * t10635 + F::cast_from(3.0_f64) * t10618 * t1555 - F::cast_from(12.0_f64) * t4415 * t4416 * t10489 - F::cast_from(36.0_f64) * t4409 * t2639 + F::cast_from(180.0_f64) * t14643 * t14649 - (t50844 + t50845 + t50847 + t50848 + t50851 + t50854 + t50882 + t50908) * t225 * t229;
    t50914
}
