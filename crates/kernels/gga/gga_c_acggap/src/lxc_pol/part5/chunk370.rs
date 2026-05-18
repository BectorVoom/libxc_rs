//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 370/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk370<F: Float>(t1313: F, t301: F, t960: F, t372: F, t530: F, t174: F, t513: F, t540: F, t1000: F, t1002: F, t1007: F, t1009: F, t1011: F, t1150: F, t335: F, t367: F, t936: F, t953: F, t976: F, t979: F, t983: F, t989: F, t995: F, t998: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1314 = t1313 * t301;
    let t1315 = t960 * t1314;
    let t1318 = t530 * t372;
    let t1319 = t960 * t1318;
    let t1322 = t174 * t513;
    let t1323 = t1322 * t301;
    let t1324 = t960 * t1323;
    let t1327 = t540 * t372;
    let t1328 = t960 * t1327;
    let t1336 = -F::new(0.21437009059034868486e-3) * t936 + F::new(0.10003937560882938627e-2) * t953 + t976 - t979 + t983 + t1150 * t1315 / F::new(16.0) + t335 * t1319 / F::new(48.0) + t335 * t1324 / F::new(48.0) + t367 * t1328 / F::new(48.0) + t989 - t995 + F::new(0.40015750243531754508e-2) * t998 - F::new(0.20007875121765877254e-2) * t1000 + F::new(0.20007875121765877254e-2) * t1002 - t1007 - F::new(0.85748036236139473944e-3) * t1009 + F::new(0.42874018118069736972e-3) * t1011;
    (t1314, t1315, t1318, t1319, t1322, t1323, t1324, t1327, t1328, t1336)
}
