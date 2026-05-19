//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1082/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1082<F: Float>(t12995: F, t3671: F, t140: F, t3693: F, t1222: F, t10326: F, t1225: F, t1012: F, t1235: F, t1238: F, t1261: F, t12933: F, t12938: F, t12942: F, t12945: F, t12949: F, t12953: F, t12956: F, t12960: F, t12964: F, t12967: F, t12972: F, t12976: F, t12979: F, t12985: F, t12988: F, t12991: F, t3663: F, t3667: F, t3674: F, t3711: F, t3714: F) -> F {
    let t12996 = t3671 * t12995;
    let t12998 = t140 * t3693;
    let t12999 = t1222 * t12998;
    let t13001 = t1225 * t10326;
    let t13002 = t1012 * t13001;
    let t13005 = F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t12933 - F::cast_from(0.7145669686344956162e-3_f64) * t3711 * t12938 + F::cast_from(0.42874018118069736972e-3_f64) * t12942 + F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t12945 - F::cast_from(0.42874018118069736972e-3_f64) * t12949 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t12953 + F::cast_from(0.85748036236139473944e-3_f64) * t12956 * t3714 + F::cast_from(0.57165357490759649295e-3_f64) * t12960 - F::cast_from(0.42874018118069736972e-3_f64) * t12964 + F::cast_from(0.12862205435420921092e-2_f64) * t12967 * t3674 - F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t12972 - F::cast_from(0.64311027177104605458e-3_f64) * t12976 * t1238 - F::cast_from(0.85748036236139473944e-3_f64) * t12979 - F::cast_from(0.64311027177104605458e-3_f64) * t3667 * t3663 + F::cast_from(0.14291339372689912324e-3_f64) * t12985 - F::cast_from(0.12862205435420921092e-2_f64) * t12988 * t12991 + F::cast_from(0.85748036236139473944e-3_f64) * t12996 - t12999 / F::new(144.0) - t1222 * t13002 / F::new(288.0);
    t13005
}
