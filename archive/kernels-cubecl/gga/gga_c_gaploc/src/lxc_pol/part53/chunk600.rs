//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 600/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk600<F: Float>(t10513: F, t10533: F, t10532: F, t3380: F, t549: F, t1429: F, t2365: F, t7893: F, t4391: F, t3395: F, t589: F, t587: F) -> (F, F, F, F) {
    let t10534 = t10533 * t10513;
    let t10536 = F::cast_from(0.27606906686822939767e2_f64) * t10532 * t10534;
    let t10537 = t549 * t3380;
    let t10538 = t1429 * t10537;
    let t10539 = F::cast_from(0.29792074959875355558e-1_f64) * t10538;
    let t10540 = t2365 * t7893;
    let t10541 = t4391 * t10540;
    let t10542 = F::cast_from(0.29792074959875355558e-1_f64) * t10541;
    let t10543 = t589 * t3395;
    let t10544 = t587 * t10543;
    (t10536, t10539, t10542, t10544)
}
