//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 418/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk418<F: Float>(t1184: F, t209: F, t1194: F, t1465: F, t4461: F, t465: F, t479: F, t198: F, t2184: F, t1193: F, t1180: F, t1189: F, t194: F, t498: F, t325: F, t892: F) -> (F, F, F, F, F, F, F, F) {
    let t4522 = t1184 * t209;
    let t4544 = t1465 * t1194;
    let t4555 = t465 * t4461;
    let t4556 = t4555 * t479;
    let t4558 = t2184 * t198;
    let t4559 = t1193 * t4558;
    let t4569 = t1180 * t1189;
    let t4585 = t194 * t498;
    let t4601 = t892 * t325;
    (t4522, t4544, t4555, t4556, t4559, t4569, t4585, t4601)
}
