//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 504/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk504<F: Float>(t10513: F, t10533: F, t10532: F, t3380: F, t549: F, t1429: F, t2365: F, t7893: F, t4391: F, t3395: F, t589: F, t587: F, t6514: F, t986: F, t544: F, t2386: F) -> (F, F, F, F, F, F, F, F) {
    let t10534 = t10533 * t10513;
    let t10536 = 0.27606906686822939767e2 * t10532 * t10534;
    let t10537 = t549 * t3380;
    let t10538 = t1429 * t10537;
    let t10539 = 0.29792074959875355558e-1 * t10538;
    let t10540 = t2365 * t7893;
    let t10541 = t4391 * t10540;
    let t10542 = 0.29792074959875355558e-1 * t10541;
    let t10543 = t589 * t3395;
    let t10544 = t587 * t10543;
    let t10545 = 0.25561950635947166451e0 * t10544;
    let t10546 = t6514 * t986;
    let t10547 = t544 * t10546;
    let t10549 = 0.25025342966295298669e1 * t10547 * t2386;
    (t10536, t10538, t10539, t10541, t10542, t10544, t10545, t10549)
}
