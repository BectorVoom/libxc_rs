//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1378/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1378(t33660: f64, t33671: f64, t33674: f64, t33682: f64, t33690: f64, t33694: f64, t33697: f64, t33680: f64, t33687: f64, t33692: f64, t36659: f64, t33701: f64) -> (f64, f64) {
    let t36660 = 0.50680539737635041234e-3_f64 * t33660;
    let t36661 = 0.52278590312710514777e-10_f64 * t33671;
    let t36662 = 0.1011909669415296852e-6_f64 * t33674;
    let t36664 = 0.2318836277704281739e-4_f64 * t33682;
    let t36666 = 0.14732367666458600006e-8_f64 * t33690;
    let t36668 = 0.18007519776492267795e-6_f64 * t33694;
    let t36669 = 0.43284943850479925795e-3_f64 * t33697;
    let t36670 = -t36659 + t36660 - t36661 + t36662 - 0.24457736545138888892e-4_f64 * t33680 + t36664 - 0.24457736545138888892e-4_f64 * t33687 + t36666 + 0.5691280480400994668e-7_f64 * t33692 - t36668 + t36669;
    let t36671 = 0.43440462632258606772e-4_f64 * t33701;
    (t36670, t36671)
}
