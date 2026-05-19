//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 812/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk812<F: Float>(t783: F, t788: F, t8279: F, t2547: F, t6118: F, t1248: F, t295: F, t1256: F, t305: F, t2376: F, t818: F, t1004: F, t1275: F) -> (F, F, F, F, F, F) {
    let t8282 = F::cast_from(0.11643651550782197811e-1_f64) * t783 * t8279 * t788;
    let t8284 = F::cast_from(0.25610080155860322884e0_f64) * t6118 * t2547;
    let t8319 = t295 * t1248;
    let t8340 = t305 * t1256;
    let t8355 = t2376 * t818;
    let t8358 = t1004 * t1275;
    (t8282, t8284, t8319, t8340, t8355, t8358)
}
