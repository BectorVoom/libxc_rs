//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3136/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3136(t50846: f64, t50848: f64, t50853: f64, t63911: f64, t63914: f64, t63918: f64, t63921: f64, t63924: f64, t63927: f64, t63930: f64, t63933: f64, t63936: f64, t63939: f64) -> f64 {
    let t64916 = -5.0_f64 / 27.0_f64 * t63911 - 2.0_f64 / 27.0_f64 * t63914 + 14.0_f64 / 81.0_f64 * t63918 + t63921 / 9.0_f64 + t63924 / 18.0_f64 + t63927 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t63930 - 8.0_f64 / 9.0_f64 * t63933 - t63936 - 4.0_f64 * t63939 + 80.0_f64 / 81.0_f64 * t50846 + 2.0_f64 / 9.0_f64 * t50848 - 20.0_f64 / 27.0_f64 * t50853;
    t64916
}
