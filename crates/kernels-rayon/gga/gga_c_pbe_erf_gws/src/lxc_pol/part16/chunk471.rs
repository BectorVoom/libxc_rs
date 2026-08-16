//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 471/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk471(t1671: f64, t1786: f64, t1877: f64, t1929: f64, t1365: f64, t153: f64, t274: f64, t542: f64, t745: f64, t1452: f64, t156: f64, t1596: f64, t1598: f64, t1601: f64, t1602: f64, t1605: f64, t1608: f64, t1611: f64, t1613: f64, t168: f64, t242: f64, t245: f64) -> (f64, f64, f64, f64) {
    let t1931 = t1671 + t1786 + t1877 + t1929;
    let t1937 = 0.13287210228946179141e1_f64 * t153 * t1365 * t274;
    let t1939 = t153 * t542 * t745;
    let t1944 = -t1596 + 0.16752564107100880375e0_f64 * t1598 + t1601 - 0.83762820535504401876e-1_f64 * t1602 * t242 - 0.16752564107100880375e0_f64 * t1605 - t1608 - t1611 + 0.39794582218349216586e-1_f64 * t1613 - 0.11938374665504764976e-1_f64 * t168 * t245 * t1931 + t1937 - 0.11389037339096724978e1_f64 * t1939 + 0.42708890021612718669e0_f64 * t153 * t156 * t1452;
    (t1931, t1937, t1939, t1944)
}
