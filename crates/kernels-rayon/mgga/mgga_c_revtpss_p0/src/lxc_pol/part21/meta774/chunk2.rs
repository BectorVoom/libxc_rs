//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2752/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2752(t10777: f64, t10779: f64, t2749: f64, t50412: f64, t14686: f64, t837: f64, t40593: f64, t4452: f64, t14671: f64, t2646: f64, t4343: f64, t836: f64) -> (f64, f64, f64, f64, f64) {
    let t50628 = t10777 * t10779 * t50412 * t2749;
    let t50632 = t10777 * t14686 * t50412 * t837;
    let t50634 = t40593 * t4452;
    let t50643 = t10777 * t14686 * t14671 * t2646;
    let t50649 = t4343 * t836;
    (t50628, t50632, t50634, t50643, t50649)
}
