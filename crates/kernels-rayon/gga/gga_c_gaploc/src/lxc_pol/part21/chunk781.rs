//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 781/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk781(t2365: f64, t5398: f64, t7390: f64, t5580: f64, t824: f64, t161: f64, t165: f64, t1880: f64, t969: f64, t2028: f64, t2033: f64, t2673: f64, t2676: f64, t5676: f64, t5694: f64, t7336: f64, t7340: f64, t7346: f64, t7349: f64, t7352: f64, t7356: f64, t7360: f64, t7364: f64, t7366: f64, t7373: f64, t7376: f64, t7380: f64, t7385: f64, t7388: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7391 = t2365 * t5398;
    let t7392 = t7390 * t7391;
    let t7394 = t5580 * t824;
    let t7396 = t161 * t165 * t1880;
    let t7397 = t969 * t7396;
    let t7398 = t7394 * t7397;
    let t7402 = -0.92686455430723328401e-1_f64 * t2033 * t7336 - 0.79445533226334281486e-1_f64 * t7340 * t2028 + 0.95857314884801874192e-1_f64 * t7346 - 0.25561950635947166452e0_f64 * t7349 + 0.51123901271894332903e0_f64 * t7352 - 0.51123901271894332903e0_f64 * t7356 + 0.8520650211982388817e-1_f64 * t7360 - 0.8520650211982388817e-1_f64 * t7364 + 0.79445533226334281486e-1_f64 * t2033 * t7366 + 0.79445533226334281486e-1_f64 * t5676 * t2676 - 0.29792074959875355558e-1_f64 * t7373 + 0.19171462976960374838e0_f64 * t7376 - 0.42603251059911944086e-1_f64 * t7380 - 0.19171462976960374838e0_f64 * t7385 + 0.19171462976960374838e0_f64 * t7388 + 0.29792074959875355558e-1_f64 * t7392 - 0.19171462976960374838e0_f64 * t7398 + 0.92686455430723328401e-1_f64 * t2673 * t5694;
    (t7391, t7392, t7394, t7396, t7398, t7402)
}
