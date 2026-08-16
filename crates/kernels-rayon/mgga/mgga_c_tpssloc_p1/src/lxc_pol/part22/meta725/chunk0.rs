//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2378/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2378(t21131: f64, t699: f64, t21135: f64, t21139: f64, t21119: f64, t5705: f64, t896: f64, t13634: f64, t13637: f64, t21510: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68500 = t699 * t21131;
    let t68502 = t699 * t21135;
    let t68504 = t699 * t21139;
    let t68506 = t699 * t21119;
    let t68508 = t5705 * t896;
    let t68509 = t13634 * t68508;
    let t68511 = t13637 * t68508;
    let t68513 = t21510 * t607;
    (t68500, t68502, t68504, t68506, t68509, t68511, t68513)
}
