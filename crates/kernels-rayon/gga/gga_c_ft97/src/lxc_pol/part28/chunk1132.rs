//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1132/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1132(t139240: f64, t34814: f64, t139514: f64, t3188: f64, t23671: f64, t5899: f64, t147656: f64, t23667: f64, t23657: f64, t27081: f64, t32979: f64, t148280: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t148334 = t139240 * t34814;
    let t148336 = t139514 * t3188;
    let t148338 = t5899 * t23671 * t148336;
    let t148342 = t5899 * t23667 * t147656;
    let t148346 = t23657 * t23671 * t32979 * t27081;
    let t148349 = t23657 * t23667 * t148280;
    (t148334, t148336, t148338, t148342, t148346, t148349)
}
