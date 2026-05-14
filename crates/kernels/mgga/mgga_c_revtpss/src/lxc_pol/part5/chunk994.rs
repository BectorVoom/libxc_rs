//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 994/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk994<F: Float>(t2398: F, t4305: F, t177: F, t4392: F, t762: F, t2626: F, t4398: F, t10439: F, t162: F, t2516: F, t2496: F, t2619: F, t4302: F, t4186: F, t750: F, t706: F) -> (F, F, F, F, F, F, F, F) {
    let t14317 = 8.0 * t2398 * t4305;
    let t14322 = t4392 * t177;
    let t14324 = 0.11696447245269292414e1 * t14322 * t762;
    let t14328 = t4398 * t2626;
    let t14330 = t10439 * t162;
    let t14334 = t4398 * t2516;
    let t14336 = t4398 * t2496;
    let t14339 = t4302 * t2619;
    let t14341 = t750 * t4186;
    let t14343 = 8.0 * t706 * t14341;
    (t14317, t14324, t14328, t14330, t14334, t14336, t14339, t14343)
}
