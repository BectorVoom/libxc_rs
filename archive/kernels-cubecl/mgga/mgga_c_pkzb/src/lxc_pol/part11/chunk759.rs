//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 759/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk759<F: Float>(t6404: F, t66: F, t2370: F, t824: F, t2029: F, t919: F, t1485: F, t178: F, t405: F, t404: F, t67: F, t931: F) -> (F, F, F, F, F, F) {
    let t6405 = t66 * t6404;
    let t6411 = t2370 * t824;
    let t6416 = t919 * t2029;
    let t6428 = t178 * t1485 * t405;
    let t6430 = F::cast_from(0.63517063878621832551e-4_f64) * t404 * t6428;
    let t6431 = t67 * t931;
    (t6405, t6411, t6416, t6428, t6430, t6431)
}
