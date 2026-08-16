//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 674/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk674(t7580: f64, t7583: f64, t1075: f64, t2140: f64, t688: f64, t7579: f64) -> (f64, f64, f64) {
    let t7584 = t7580 * t7583;
    let t7587 = t688 * t1075 * t2140;
    let t7589 = t688 * t7579;
    (t7584, t7587, t7589)
}
