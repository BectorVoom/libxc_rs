//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1308/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1308(t23265: f64, t6547: f64, t23030: f64, t23208: f64, t82120: f64, t82123: f64, t82126: f64, t82129: f64, t82131: f64, t82135: f64, t82138: f64, t82141: f64, t82143: f64) -> f64 {
    let t82145 = t6547 * t23265;
    let t82147 = t23030 * t23208;
    let t82149 = 0.49348022005446793095e-1_f64 * t82120 - t82123 - 0.24674011002723396548e-1_f64 * t82126 + 0.49348022005446793095e-1_f64 * t82129 - 0.57572692339687925277e-1_f64 * t82131 + 0.24674011002723396547e-1_f64 * t82135 - 0.49348022005446793095e-1_f64 * t82138 + 0.14804406601634037928e0_f64 * t82141 + 0.57572692339687925277e-1_f64 * t82143 + 0.11514538467937585055e0_f64 * t82145 - 0.78134368175290755733e-1_f64 * t82147;
    t82149
}
