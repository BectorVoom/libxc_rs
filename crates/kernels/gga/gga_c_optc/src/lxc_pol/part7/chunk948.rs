//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 948/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk948<F: Float>(t3092: F, t9044: F, t914: F, t3105: F, t4298: F, t8975: F, t8969: F, t1150: F, t1162: F, t1170: F, t3234: F, t4457: F, t4464: F, t9007: F, t9010: F, t9013: F, t9016: F, t9019: F, t9022: F, t9026: F, t9031: F, t9035: F, t9038: F, t9041: F) -> (F, F, F) {
    let t9045 = t3092 * t9044;
    let t9046 = t914 * t9045;
    let t9049 = t4298 * t3105;
    let t9050 = t9049 * t8975;
    let t9053 = t9049 * t8969;
    let t9056 = F::new(0.11360101276506094136e1) * t9007 - F::new(0.4395493670620718481e3) * t9010 + F::new(0.1169609647897054359e2) * t9013 + F::new(0.8790987341241436962e3) * t9016 - F::new(0.15486228121497046737e2) * t9019 - F::new(0.75734008510040627575e0) * t9022 + F::new(0.11360101276506094136e1) * t1150 * t9026 + F::new(0.5848048239485271795e1) * t1170 * t9031 - F::new(0.389869882632351453e1) * t9035 + F::new(0.4645868436449114021e2) * t9038 - F::new(0.2339219295794108718e2) * t3234 * t9041 - F::new(0.17386322979577515709e0) * t1162 * t9046 + F::new(0.8790987341241436962e3) * t4457 * t9050 - F::new(0.4395493670620718481e3) * t4464 * t9053;
    (t9045, t9049, t9056)
}
