//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2821/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2821(t2394: f64, t890: f64, t198: f64, t14353: f64, t40167: f64, t40171: f64, t40184: f64, t4541: f64, t4553: f64, t50874: f64, t50875: f64, t50876: f64, t50879: f64, t50881: f64, t50884: f64) -> (f64, f64) {
    let t51775 = t890 * t2394;
    let t51780 = t198 * t2394;
    let t51786 = 18.0_f64 * t14353 * t2394 * t4541 + 18.0_f64 * t4553 * t51780 + t40167 - t40171 - t40184 + t50874 - t50875 + t50876 + t50879 + t50881 + t50884;
    (t51775, t51786)
}
