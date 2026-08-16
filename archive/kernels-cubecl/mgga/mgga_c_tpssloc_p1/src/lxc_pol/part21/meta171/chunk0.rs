//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1096/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1096<F: Float>(t182: F, t4095: F, t145: F, t4094: F, t185: F, t1472: F, t751: F, t1409: F) -> (F, F, F, F, F) {
    let t4097 = F::cast_from(0.19751673498613801407e-1_f64) * t4095 * t182;
    let t4098 = t145 * t4094;
    let t4099 = t4098 * t185;
    let t4100 = t1472 * t751;
    let t4101 = t751 * t1409;
    (t4097, t4098, t4099, t4100, t4101)
}
