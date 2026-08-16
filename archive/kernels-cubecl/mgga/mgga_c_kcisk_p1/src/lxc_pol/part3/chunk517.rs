//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 517/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk517<F: Float>(t4175: F, t486: F, t1297: F, t391: F, t494: F, t79: F, t499: F, t493: F, t1512: F, t1516: F, t3732: F, t498: F) -> (F, F, F, F, F, F, F, F) {
    let t4176 = t486 * t4175;
    let t4180 = F::cast_from(1.0_f64) / t391 / t494 / t1297;
    let t4181 = t4180 * t79;
    let t4182 = t4181 * t499;
    let t4183 = t493 * t4182;
    let t4185 = t1512 * t1516;
    let t4186 = t493 * t4185;
    let t4188 = t499 * t3732;
    let t4189 = t498 * t4188;
    (t4176, t4181, t4182, t4183, t4185, t4186, t4188, t4189)
}
