//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 387/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk387(t536: f64, t700: f64, t1383: f64, t148: f64, t1472: f64, t168: f64, t270: f64, t703: f64, t738: f64, t732: f64, t735: f64, t155: f64, t266: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1605 = t536 * t700;
    let t1608 = 0.83762820535504401876e-1_f64 * t148 * t1383;
    let t1611 = 0.53059442957798955448e-1_f64 * t168 * t1472 * t270;
    let t1613 = t168 * t703 * t738;
    let t1615 = t732 * t735;
    let t1617 = t266 * t155;
    (t1605, t1608, t1611, t1613, t1615, t1617)
}
