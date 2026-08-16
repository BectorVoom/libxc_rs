//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 910/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk910(t8392: f64, t8520: f64, t1570: f64, t1580: f64, t1557: f64, t1882: f64, t8570: f64, t8529: f64, t1786: f64, t1825: f64, t3281: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38681 = t8392 * t8520;
    let t38688 = t1570 * t1580;
    let t38693 = t1557 * t1580;
    let t38698 = t1882 * t8570;
    let t38700 = t1882 * t8529;
    let t38711 = t1786 * t1825;
    let t38732 = t3281 * t494;
    (t38681, t38688, t38693, t38698, t38700, t38711, t38732)
}
