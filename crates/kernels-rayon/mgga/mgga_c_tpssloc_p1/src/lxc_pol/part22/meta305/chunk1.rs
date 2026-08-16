//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1473/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1473(t225: f64, t4658: f64, t4553: f64, t4559: f64, t4555: f64, t14506: f64, t3199: f64) -> (f64, f64, f64, f64, f64) {
    let t14529 = t4658 * t225;
    let t14545 = t4553 * t225;
    let t14552 = t4559 * t225;
    let t14555 = t4555 * t225;
    let t14608 = t14506 * t3199;
    (t14529, t14545, t14552, t14555, t14608)
}
