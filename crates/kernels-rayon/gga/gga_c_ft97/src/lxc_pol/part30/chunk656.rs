//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 656/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk656(t1701: f64, t4125: f64, t6027: f64, t27494: f64, t811: f64, t820: f64, t992: f64, t704: f64, t25069: f64, t4113: f64, t22511: f64, t7004: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28540 = t1701 * t6027 * t4125;
    let t28544 = t1701 * t27494 * t811;
    let t28547 = t992 * t820;
    let t28548 = t704 * t28547;
    let t28552 = t4113 * t25069;
    let t28557 = t7004 * t22511;
    (t28540, t28544, t28547, t28548, t28552, t28557)
}
