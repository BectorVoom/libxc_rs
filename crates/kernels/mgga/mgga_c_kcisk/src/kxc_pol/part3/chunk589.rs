//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 589/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk589<F: Float>(t2029: F, t5520: F, t1994: F, t5075: F, t5078: F, t5080: F, t5178: F, t5189: F, t5197: F, t5201: F, t5206: F, t5432: F, t5440: F, t5445: F, t795: F, t5355: F) -> (F, F) {
    let t5521 = t5520 * t2029;
    let t5524 = 0.15476481481481481481e-2 * t5075 + 0.23214722222222222222e-2 * t5078 + 0.23214722222222222222e-2 * t5080 + 0.17411041666666666666e-2 * t5178 + t5432 * t795 - 0.23214722222222222222e-2 * t5189 + 0.15476481481481481481e-2 * t5197 - 0.23214722222222222222e-2 * t5201 + 0.193e0 * t1994 * t5440 + 0.74498e-1 * t5445 * t5440 - 0.38691203703703703703e-3 * t5206 - 0.193e0 * t1994 * t5521;
    let t5525 = t5355 + t5524;
    (t5521, t5525)
}
