//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1345/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1345<F: Float>(t19790: F, t921: F, t19789: F, t22947: F, t2530: F, t538: F, t6191: F, t6194: F, t20430: F, t20462: F, t20464: F, t20468: F, t20471: F, t20475: F, t20479: F, t20484: F, t20488: F, t20490: F, t25088: F, t529: F, t535: F, t6425: F, t8059: F, t948: F) -> (F,) {
    let t25397 = t19790 * t921;
    let t25399 = t22947 * t19789 * t25397;
    let t25405 = t6191 * t538 * t2530 * t6194;
    let t25406 = 0.43371823197556470519e-3 * t25405;
    let t25414 = 0.17465477326173296717e-1 * t20462 + 0.19043987679069580389e-1 * t20464 + 0.52396431978519890151e-1 * t20468 - 0.53665922966605306603e-2 * t20471 - 0.14636160809074174528e-2 * t20475 + 0.11557628986739024751e0 * t20479 + 0.59329162131926993722e1 * t20484 + 0.25426783770825854452e1 * t20488 - 0.41530324072742201648e-1 * t25399 - 0.43341108700271342816e-1 * t20430 * t948 - t25406 + 0.7801399566048841707e0 * t6425 * t8059 - 0.27439371595564631661e-1 * t535 * t529 * t538 * t25088 + 0.87816964854445047168e-1 * t20490;
    (t25414,)
}
