//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 857/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk857(t35593: f64, t35632: f64, t35692: f64, t35742: f64, t35516: f64, t675: f64, t263: f64, t193: f64, t6743: f64, t7150: f64, t1091: f64, t1425: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35744 = t35593 + t35632 + t35692 + t35742;
    let t35751 = t675 * t35516;
    let t35752 = t35751 * t263;
    let t35753 = t193 * t35752;
    let t35757 = t6743 * t7150;
    let t35760 = t1425 * t1091;
    (t35744, t35751, t35752, t35753, t35757, t35760)
}
