//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 302/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk302<F: Float>(t1602: F, t305: F, t1361: F, t797: F, t1365: F, t838: F, t1369: F, t326: F, t1587: F, t27: F, t29: F, t847: F) -> (F, F, F, F, F) {
    let t1603 = t305 * t1602;
    let t1605 = t797 * t1361;
    let t1607 = t838 * t1365;
    let t1609 = t326 * t1369;
    let t1612 = t1587 * t29 * t27;
    let t1614 = F::new(5.0) / F::new(18.0) * t1612 + t847;
    (t1603, t1605, t1607, t1609, t1614)
}
