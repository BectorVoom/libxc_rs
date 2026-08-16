//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 345/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk345<F: Float>(t1167: F, t179: F, t932: F, t1220: F, t1224: F, t1230: F, t1238: F, t385: F, t388: F, t404: F, t407: F, t906: F, t918: F, t929: F) -> (F, F) {
    let t1242 = t179 * t932 * t1167;
    let t1245 = -t1220 * t388 / F::cast_from(36.0_f64) + t906 - t385 * t1224 / F::cast_from(96.0_f64) + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t1230 - F::cast_from(0.11433071498151929859e-2_f64) * t1238 * t407 + t929 - F::cast_from(0.42874018118069736972e-3_f64) * t404 * t1242;
    (t1242, t1245)
}
