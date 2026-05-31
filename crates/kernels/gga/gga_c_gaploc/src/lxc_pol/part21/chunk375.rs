//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 375/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk375<F: Float>(t293: F, t711: F, t291: F, t1233: F, t1236: F, t1685: F, t1227: F, t286: F, t458: F, t714: F, t1237: F, t1681: F, t1687: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t1691 = F::cast_from(1.0_f64) / t711 / t293;
    let t1692 = t291 * t1691;
    let t1693 = t1692 * t1233;
    let t1695 = t1236 * t1685 * pi;
    let t1699 = t1227 * t286 * t458;
    let t1700 = t714 * t1699;
    let t1702 = F::cast_from(63.0_f64) / F::cast_from(256.0_f64) * t1681 - F::cast_from(49.0_f64) / F::cast_from(8192.0_f64) * t1237 * t1687 + F::cast_from(49.0_f64) / F::cast_from(24576.0_f64) * t1693 * t1695 - F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t1700;
    (t1691, t1692, t1695, t1699, t1700, t1702)
}
