//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2886/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2886<F: Float>(t2: F, t580: F, t6084: F, t19049: F, t4729: F, t23649: F, t3022: F, t19023: F, t4719: F, t23457: F, t23478: F, t689: F) -> (F, F, F, F, F, F) {
    let t77481 = F::new(3.0) * t6084 * t2 * t580;
    let t77492 = F::cast_from(0.17544670867903938621e1_f64) * t19049 * t4729;
    let t77494 = F::cast_from(0.10254018858216406658e4_f64) * t3022 * t23649;
    let t77496 = F::cast_from(0.17544670867903938621e1_f64) * t4719 * t19023;
    let t77498 = F::cast_from(0.35089341735807877242e1_f64) * t3022 * t23457;
    let t77499 = t689 * t23478;
    (t77481, t77492, t77494, t77496, t77498, t77499)
}
