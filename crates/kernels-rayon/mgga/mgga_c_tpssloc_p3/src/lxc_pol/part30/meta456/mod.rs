//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1727;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta456(t23069: f64, t805: f64, t243: f64, t598: f64, t213: f64, t6584: f64, t6604: f64, t6606: f64, t1891: f64, t22822: f64, t133: f64, t6601: f64, t6590: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23070, t23075, t23076, t23077, t23078, t23083) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1727(t23069, t805, t243, t598, t213, t6584, t6604);
        let (t23084, t23093, t23096, t23097) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1728(t23083, t6606, t1891, t22822, t133, t6601, t6590, t6604);
    (t23070, t23075, t23076, t23077, t23078, t23083, t23084, t23093, t23096, t23097)
}
