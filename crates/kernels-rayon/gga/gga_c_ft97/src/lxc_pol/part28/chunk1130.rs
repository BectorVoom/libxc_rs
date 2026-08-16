//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1130/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1130(t23649: f64, t34832: f64, t23657: f64, t27147: f64, t32924: f64, t9432: f64, t23671: f64, t34843: f64, t379: f64, t139214: f64, t139224: f64, t26950: f64, t32897: f64) -> (f64, f64, f64, f64) {
    let t148311 = t23649 * t34832;
    let t148315 = t23657 * t9432 * t32924 * t27147;
    let t148319 = t23657 * t23671 * t34843 * t379;
    let t148323 = t32897 * t139224 * t139214 * t26950;
    (t148311, t148315, t148319, t148323)
}
