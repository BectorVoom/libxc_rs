//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 556/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk556(t3598: f64, t7757: f64, t3572: f64, t5668: f64, t7738: f64, t7742: f64, t7746: f64) -> (f64, f64) {
    let t7758 = t3598 * t7757;
    let t7764 = t3572 + 2.0_f64 / 9.0_f64 * t5668 - 2.0_f64 / 9.0_f64 * t7738 + 2.0_f64 / 3.0_f64 * t7742 - t7746 / 3.0_f64;
    (t7758, t7764)
}
