//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 984/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk984<F: Float>(t11002: F, t11544: F, t3269: F, t10663: F, t3579: F, t2526: F, t797: F, t3262: F, t3263: F, t2333: F, t983: F, t795: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11545 = t11002 * t11544;
    let t11546 = t3269 * t11545;
    let t11547 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t11546;
    let t11548 = t3579 * t10663;
    let t11549 = t11548 / F::cast_from(4.0_f64);
    let t11550 = t797 * t2526;
    let t11552 = t3262 * t3263 * t11550;
    let t11553 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t11552;
    let t11554 = t2333 * t983;
    let t11555 = t11554 * t795;
    (t11545, t11546, t11547, t11548, t11549, t11550, t11552, t11553, t11554, t11555)
}
