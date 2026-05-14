//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 739/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk739<F: Float>(t2365: F, t5398: F, t7390: F, t5580: F, t824: F, t161: F, t165: F, t1880: F, t969: F, t2028: F, t2033: F, t2673: F, t2676: F, t5676: F, t5694: F, t7336: F, t7340: F, t7346: F, t7349: F, t7352: F, t7356: F, t7360: F, t7364: F, t7366: F, t7373: F, t7376: F, t7380: F, t7385: F, t7388: F) -> (F, F, F, F, F, F) {
    let t7391 = t2365 * t5398;
    let t7392 = t7390 * t7391;
    let t7394 = t5580 * t824;
    let t7396 = t161 * t165 * t1880;
    let t7397 = t969 * t7396;
    let t7398 = t7394 * t7397;
    let t7402 = -0.92686455430723328401e-1 * t2033 * t7336 - 0.79445533226334281486e-1 * t7340 * t2028 + 0.95857314884801874192e-1 * t7346 - 0.25561950635947166452e0 * t7349 + 0.51123901271894332903e0 * t7352 - 0.51123901271894332903e0 * t7356 + 0.8520650211982388817e-1 * t7360 - 0.8520650211982388817e-1 * t7364 + 0.79445533226334281486e-1 * t2033 * t7366 + 0.79445533226334281486e-1 * t5676 * t2676 - 0.29792074959875355558e-1 * t7373 + 0.19171462976960374838e0 * t7376 - 0.42603251059911944086e-1 * t7380 - 0.19171462976960374838e0 * t7385 + 0.19171462976960374838e0 * t7388 + 0.29792074959875355558e-1 * t7392 - 0.19171462976960374838e0 * t7398 + 0.92686455430723328401e-1 * t2673 * t5694;
    (t7391, t7392, t7394, t7396, t7398, t7402)
}
