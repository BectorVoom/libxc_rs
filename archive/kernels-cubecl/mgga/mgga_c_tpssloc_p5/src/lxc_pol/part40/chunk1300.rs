//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1300/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1300<F: Float>(t112: F, t30465: F, t100930: F, t110240: F, t111415: F, t1401: F, t1458: F, t16521: F, t16524: F, t19534: F, t20162: F, t20173: F, t20181: F, t2180: F, t29996: F, t30180: F, t30231: F, t30250: F, t30492: F, t30495: F, t3941: F, t4072: F, t5456: F, t5493: F, t55353: F, t55388: F, t66958: F, t671: F, t8143: F, t8166: F, t8230: F, t8251: F) -> F {
    let t111674 = t30465 * t112;
    let t111683 = F::cast_from(54.0_f64) * t55353 * t8251 + F::cast_from(27.0_f64) * t20173 * t30495 + F::cast_from(27.0_f64) * t3941 * t8143 * t5493 + F::cast_from(27.0_f64) * t3941 * t2180 * t19534 + F::cast_from(27.0_f64) * t110240 * t5456 + F::cast_from(0.135e2_f64) * t1401 * t111415 + F::cast_from(27.0_f64) * t30231 * t4072 + F::cast_from(54.0_f64) * t16524 * t30250 + F::cast_from(54.0_f64) * t20173 * t30492 + F::cast_from(54.0_f64) * t3941 * t30180 * t1458 + F::cast_from(54.0_f64) * t3941 * t8230 * t4072 + F::cast_from(27.0_f64) * t16521 * t8230 + F::cast_from(0.135e2_f64) * t66958 * t2180 + F::cast_from(27.0_f64) * t100930 * t2180 + F::cast_from(0.135e2_f64) * t111674 * t671 + F::cast_from(27.0_f64) * t29996 * t20181 + F::cast_from(0.135e2_f64) * t20162 * t8143 + F::cast_from(27.0_f64) * t55388 * t8166;
    t111683
}
