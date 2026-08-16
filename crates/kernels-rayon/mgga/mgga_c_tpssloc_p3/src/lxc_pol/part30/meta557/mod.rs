//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta557(t28190: f64, t28236: f64, t533: f64, t1390: f64, t1983: f64, t25: f64, t5527: f64, t1915: f64, t1484: f64, t1530: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t28237, t28238, t28239, t28240, t28241, t28242, t28248) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1917(t28190, t28236, t533, t1390, t1983, t25, t5527, t1915, t1484, t1530);
    (t28237, t28238, t28239, t28240, t28241, t28242, t28248)
}
