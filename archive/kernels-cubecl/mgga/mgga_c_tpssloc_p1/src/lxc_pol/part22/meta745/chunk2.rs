//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2475/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2475<F: Float>(t10422: F, t21519: F, t3070: F, t10403: F, t10408: F, t10904: F, t21487: F, t49662: F, t5677: F, t61916: F, t61919: F, t61923: F, t61929: F, t61940: F, t61975: F, t61977: F, t70082: F) -> F {
    let t70404 = t3070 * t10422 * t21519;
    let t70414 = F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t10403 * t10408 * t5677 * t70082 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t61916 - t70404 / F::cast_from(1152.0_f64) - t61919 / F::cast_from(576.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t61923 + t61929 / F::cast_from(1152.0_f64) - t10904 * t21487 / F::cast_from(96.0_f64) - t49662 + t61940 / F::cast_from(1152.0_f64) - t61975 / F::cast_from(1536.0_f64) + t61977 / F::cast_from(2304.0_f64);
    t70414
}
