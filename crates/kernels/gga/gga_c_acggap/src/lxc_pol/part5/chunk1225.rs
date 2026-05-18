//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1225/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1225<F: Float>(t20432: F, t944: F, t3379: F, t5712: F, t3375: F, t6157: F, t1163: F, t1165: F, t4162: F, t5852: F, t1180: F, t1181: F, t1531: F, t1552: F, t157: F, t17216: F, t20972: F, t22383: F, t22388: F, t22397: F, t22399: F, t3462: F, t4267: F, t4643: F, t4838: F) -> (F, F) {
    let t22401 = t20432 * t944;
    let t22410 = t3379 * t5712;
    let t22417 = t3375 * t6157;
    let t22421 = t1163 * t1165 * t5852 * t4162;
    let t22424 = F::new(35.0) / F::new(432.0) * t22383 - F::new(0.85748036236139473944e-3) * t22388 + F::new(0.17149607247227894789e-2) * t1180 * t1165 * t1552 * t20972 - F::new(0.17149607247227894789e-2) * t22397 + F::new(0.80031500487063509014e-2) * t22399 + F::new(0.68598428988911579156e-2) * t3462 * t1165 * t4267 * t22401 - F::new(0.34299214494455789578e-2) * t1531 * t1181 * t4643 * t22401 + F::new(0.68598428988911579156e-2) * t22410 + F::new(0.85748036236139473944e-3) * t1180 * t1165 * t1552 * t157 * t4838 + F::new(0.42874018118069736972e-3) * t22417 + F::new(0.21437009059034868486e-3) * t22421 - F::new(35.0) / F::new(108.0) * t17216;
    (t22401, t22424)
}
