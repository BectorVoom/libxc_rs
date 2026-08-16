//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2016/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2016<F: Float>(t2085: F, t6414: F, t1352: F, t19810: F, t27078: F, t5344: F, t81047: F, t84480: F, t90889: F, t90900: F, t90903: F, t93562: F, t93572: F, t96986: F, t96989: F, t96993: F, t96997: F, t97002: F, t97007: F, t97014: F, t97017: F) -> (F, F) {
    let t102587 = t2085 * t6414;
    let t102597 = -F::cast_from(0.52089578783527170489e-1_f64) * t81047 - F::cast_from(0.16449340668482264365e-1_f64) * t96986 + F::cast_from(0.82246703342411321825e-2_f64) * t96989 + F::cast_from(0.19739208802178717238e0_f64) * t96993 + F::cast_from(0.3289868133696452873e-1_f64) * t96997 - t5344 * t102587 * t1352 - F::cast_from(2.0_f64) * t19810 * t27078 - F::cast_from(0.9869604401089358619e-1_f64) * t97002 - t90889 - F::cast_from(0.6579736267392905746e-1_f64) * t97007 - t93562 + F::cast_from(0.10417915756705434098e0_f64) * t90900 + t90903 - t93572 - F::cast_from(0.39478417604357434476e0_f64) * t97014 - F::cast_from(0.3289868133696452873e-1_f64) * t97017 - t84480;
    (t102587, t102597)
}
