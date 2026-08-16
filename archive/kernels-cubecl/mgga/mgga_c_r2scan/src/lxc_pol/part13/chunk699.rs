//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 699/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk699<F: Float>(t234: F, t5348: F, t1835: F, t712: F, t1837: F, t1831: F, t732: F, t225: F, t5317: F, t739: F, t166: F, t1726: F) -> (F, F, F, F, F) {
    let t5350 = F::cast_from(0.35089341735807877242e1_f64) * t234 * t5348;
    let t5351 = t1835 * t712;
    let t5352 = t5351 * t1837;
    let t5354 = F::cast_from(0.31168546390226634765e3_f64) * t234 * t5352;
    let t5355 = t732 * t1831;
    let t5357 = t225 * t5317;
    let t5358 = t739 * t5357;
    let t5360 = F::cast_from(0.11696447245269292414e1_f64) * t234 * t5358;
    let t5363 = t1726 * t166;
    (t5350, t5354, t5355, t5360, t5363)
}
