//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2207/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2207<F: Float>(t4158: F, t7950: F, t18190: F, t2042: F, t1459: F, t28271: F, t5795: F, t7334: F, t1518: F, t572: F, t95137: F, t26123: F, t4292: F) -> (F, F, F, F, F, F) {
    let t101632 = F::cast_from(6.0_f64) * t4158 * t7950;
    let t101634 = F::cast_from(3.0_f64) * t18190 * t2042;
    let t101640 = F::cast_from(12.0_f64) * t1459 * t28271;
    let t101642 = F::cast_from(6.0_f64) * t5795 * t7334;
    let t101645 = F::cast_from(6.0_f64) * t572 * t95137 * t1518;
    let t101648 = F::cast_from(12.0_f64) * t572 * t26123 * t4292;
    (t101632, t101634, t101640, t101642, t101645, t101648)
}
