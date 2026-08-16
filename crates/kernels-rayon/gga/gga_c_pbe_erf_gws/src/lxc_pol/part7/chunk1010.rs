//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1010/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1010(t168: f64, t5589: f64, t738: f64, t1365: f64, t1452: f64, t153: f64, t18046: f64, t274: f64, t1457: f64, t700: f64, t1383: f64, t762: f64) -> (f64, f64, f64, f64, f64) {
    let t18352 = t168 * t5589 * t738;
    let t18355 = t153 * t1365 * t1452;
    let t18359 = 0.19192636997366703204e2_f64 * t153 * t18046 * t274;
    let t18360 = t1457 * t700;
    let t18363 = 0.10051538464260528225e1_f64 * t762 * t1383;
    (t18352, t18355, t18359, t18360, t18363)
}
