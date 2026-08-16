//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3295/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3295<F: Float>(t1892: F, t6843: F, t1399: F, t1883: F, t22009: F, t46570: F, t49199: F, t49203: F, t49210: F, t5659: F, t5755: F, t74973: F, t75113: F, t75119: F, t75123: F, t75128: F, t86455: F) -> (F, F) {
    let t86506 = t1892 * t6843;
    let t86533 = t49199 - F::cast_from(0.91069445034239308177e-1_f64) * t49203 - F::cast_from(0.78059524315062264151e-2_f64) * t49210 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t22009 * t5659 - F::cast_from(0.39029762157531132074e-2_f64) * t75113 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t86455 * t1399 - F::cast_from(0.21951497276451705328e-1_f64) * t75119 - F::cast_from(0.34697458558045176418e-2_f64) * t75123 - F::cast_from(0.34697458558045176418e-2_f64) * t75128 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t74973 * t1883 + F::cast_from(0.17073386770573548589e-1_f64) * t46570;
    (t86506, t86533)
}
