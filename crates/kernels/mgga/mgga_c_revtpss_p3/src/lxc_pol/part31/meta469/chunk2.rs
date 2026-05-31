//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1716/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1716<F: Float>(t1412: F, t6816: F, t1353: F, t1394: F, t21969: F, t1392: F, t1395: F, t1877: F, t1879: F, t22223: F, t22229: F, t22237: F, t22240: F, t539: F, t541: F, t5644: F, t5650: F, t5652: F, t5655: F, t6832: F, t6837: F, t6840: F) -> F {
    let t22245 = t1412 * t6816;
    let t22246 = t22245 * t1353;
    let t22249 = t1394 * t21969;
    let t22252 = -F::cast_from(12.0_f64) * t1392 * t6837 + F::cast_from(3.0_f64) * t1392 * t6840 + F::cast_from(3.0_f64) * t1395 * t6832 + F::cast_from(6.0_f64) * t1877 * t5655 + F::cast_from(6.0_f64) * t1879 * t5644 - t22223 * t541 - F::cast_from(24.0_f64) * t22229 * t5652 + F::cast_from(60.0_f64) * t22237 * t5650 - F::cast_from(24.0_f64) * t22240 * t5650 - F::cast_from(12.0_f64) * t22246 * t5650 + F::cast_from(3.0_f64) * t22249 * t539;
    t22252
}
