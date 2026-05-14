//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 657/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk657<F: Float>(t1827: F, t732: F, t1842: F, t1810: F, t1838: F, t1826: F, t1830: F, t234: F, t1835: F, t712: F, t1837: F, t1831: F, t225: F, t5317: F, t739: F, t166: F, t1726: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5338 = t732 * t1827;
    let t5340 = t732 * t1842;
    let t5344 = t732 * t1810;
    let t5346 = t732 * t1838;
    let t5348 = t1826 * t1830;
    let t5350 = 0.35089341735807877242e1 * t234 * t5348;
    let t5351 = t1835 * t712;
    let t5352 = t5351 * t1837;
    let t5354 = 0.31168546390226634765e3 * t234 * t5352;
    let t5355 = t732 * t1831;
    let t5357 = t225 * t5317;
    let t5358 = t739 * t5357;
    let t5360 = 0.11696447245269292414e1 * t234 * t5358;
    let t5363 = t1726 * t166;
    (t5338, t5340, t5344, t5346, t5350, t5354, t5355, t5360, t5363)
}
