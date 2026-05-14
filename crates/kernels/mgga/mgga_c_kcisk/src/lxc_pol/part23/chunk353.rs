//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 353/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk353<F: Float>(t1459: F, t1520: F, t1161: F, t1165: F, t512: F) -> (F, F, F, F, F) {
    let t1521 = t1459 * t1520;
    let t1522 = 0.17123333333333333333e-1 * t1161;
    let t1524 = -t1522 - 0.17123333333333333333e-1 * t1165;
    let t1527 = t512 * t512;
    let t1528 = 1.0 / t1527;
    (t1521, t1522, t1524, t1527, t1528)
}
