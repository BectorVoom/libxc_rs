//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 922/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk922(t41: f64, t7908: f64, t2523: f64, t700: f64, t1383: f64, t992: f64, t153: f64, t156: f64, t168: f64, t242: f64, t245: f64, t5580: f64, t5585: f64, t5592: f64, t7976: f64, t7981: f64, t8038: f64, t8042: f64, t8047: f64, t8050: f64, t8051: f64) -> (f64, f64) {
    let t8053 = t41 * t7908;
    let t8057 = 0.16752564107100880375e0_f64 * t2523 * t700;
    let t8058 = t992 * t1383;
    let t8061 = -0.11938374665504764976e-1_f64 * t168 * t245 * t7976 + 0.13287210228946179141e1_f64 * t7981 + 0.42708890021612718669e0_f64 * t153 * t156 * t8038 - 0.16752564107100880375e0_f64 * t8042 - 0.56945186695483624892e0_f64 * t5580 - t8047 + t8050 + 0.16752564107100880375e0_f64 * t8051 - 0.83762820535504401876e-1_f64 * t8053 * t242 - t8057 - 0.83762820535504401876e-1_f64 * t8058 + 0.26574420457892358282e1_f64 * t5585 + t5592;
    (t8053, t8061)
}
