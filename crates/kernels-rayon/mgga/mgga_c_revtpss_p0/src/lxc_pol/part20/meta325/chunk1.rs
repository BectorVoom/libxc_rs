//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1239/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1239(t10326: f64, t1225: f64, t1012: f64, t1222: f64, t1235: f64, t1238: f64, t1261: f64, t12933: f64, t12938: f64, t12942: f64, t12945: f64, t12949: f64, t12953: f64, t12956: f64, t12960: f64, t12964: f64, t12967: f64, t12972: f64, t12976: f64, t12979: f64, t12985: f64, t12988: f64, t12991: f64, t12996: f64, t12999: f64, t3663: f64, t3667: f64, t3674: f64, t3711: f64, t3714: f64) -> (f64, f64) {
    let t13001 = t1225 * t10326;
    let t13002 = t1012 * t13001;
    let t13005 = 0.42874018118069736972e-3_f64 * t3711 * t12933 - 0.7145669686344956162e-3_f64 * t3711 * t12938 + 0.42874018118069736972e-3_f64 * t12942 + 0.71456696863449561621e-3_f64 * t1261 * t12945 - 0.42874018118069736972e-3_f64 * t12949 + 0.42874018118069736972e-3_f64 * t3711 * t12953 + 0.85748036236139473944e-3_f64 * t12956 * t3714 + 0.57165357490759649295e-3_f64 * t12960 - 0.42874018118069736972e-3_f64 * t12964 + 0.12862205435420921092e-2_f64 * t12967 * t3674 - 0.21437009059034868486e-3_f64 * t1235 * t12972 - 0.64311027177104605458e-3_f64 * t12976 * t1238 - 0.85748036236139473944e-3_f64 * t12979 - 0.64311027177104605458e-3_f64 * t3667 * t3663 + 0.14291339372689912324e-3_f64 * t12985 - 0.12862205435420921092e-2_f64 * t12988 * t12991 + 0.85748036236139473944e-3_f64 * t12996 - t12999 / 144.0_f64 - t1222 * t13002 / 288.0_f64;
    (t13001, t13005)
}
