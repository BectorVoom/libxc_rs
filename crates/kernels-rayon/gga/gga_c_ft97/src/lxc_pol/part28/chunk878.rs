//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 878/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk878(t32945: f64, t32961: f64, t32978: f64, t34851: f64, t34856: f64, t34921: f64, t34925: f64, t34929: f64, t34933: f64, t34937: f64, t34941: f64, t34945: f64) -> f64 {
    let t34946 = t32945 + t34851 / 6.0_f64 + t34856 - t34921 / 2.0_f64 - t32961 - 2.0_f64 / 3.0_f64 * t34925 - 6.0_f64 * t34929 + 4.0_f64 * t34933 + t32978 + t34937 / 3.0_f64 + 2.0_f64 * t34941 - t34945;
    t34946
}
