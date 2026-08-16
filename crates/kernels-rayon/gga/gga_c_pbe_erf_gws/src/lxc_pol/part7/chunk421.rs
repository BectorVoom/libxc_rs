//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 421/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk421(t1769: f64, t571: f64, t11: f64, t1755: f64, t1756: f64, t1761: f64, t1767: f64, t173: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t1770 = t571 * t1769;
    let t1771 = t11 * t1770;
    let t1773 = -t1755 - 0.12594444444444444445e-2_f64 * t1756 + 0.12594444444444444445e-2_f64 * t1761 - 0.37783333333333333334e-2_f64 * t1767 + 0.18891666666666666667e-2_f64 * t1771;
    let t1774 = t173 * t1773;
    let t1775 = t1774 * t184;
    (t1770, t1771, t1773, t1774, t1775)
}
