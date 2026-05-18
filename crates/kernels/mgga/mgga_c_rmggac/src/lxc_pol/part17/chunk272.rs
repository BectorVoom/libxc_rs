//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 272/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk272<F: Float>(t1502: F, t221: F, t476: F, t589: F, t209: F, t1228: F, t612: F, t1231: F, t219: F, t6: F, t446: F, t1392: F, t489: F, t490: F) -> (F, F, F, F, F, F, F, F) {
    let t1503 = t221 * t1502;
    let t1508 = t589 * t476;
    let t1509 = t1508 * t209;
    let t1510 = t221 * t1509;
    let t1513 = t1228 * t612;
    let t1515 = t1231 * t219;
    let t1516 = t6 * t589;
    let t1518 = t1515 * t1516 * t446;
    let t1522 = t489 * t490 * t1392;
    (t1503, t1508, t1510, t1513, t1515, t1516, t1518, t1522)
}
