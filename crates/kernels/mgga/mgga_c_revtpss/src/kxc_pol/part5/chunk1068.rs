//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1068/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1068<F: Float>(t14328: F, t14334: F, t14336: F, t14339: F, t5819: F, t750: F, t2611: F, t2398: F, t5999: F, t5825: F, t706: F, t4305: F, t4311: F, t14363: F, t162: F, t18298: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18535 = 0.23392894490538584828e1 * t14328;
    let t18536 = 0.11696447245269292414e1 * t14334;
    let t18537 = 0.34631718211362927517e2 * t14336;
    let t18538 = 0.48830526149350786811e-3 * t14339;
    let t18539 = t750 * t5819;
    let t18540 = t2611 * t18539;
    let t18541 = 12.0 * t18540;
    let t18543 = 4.0 * t2398 * t5999;
    let t18544 = t750 * t5825;
    let t18545 = t706 * t18544;
    let t18546 = 4.0 * t18545;
    let t18547 = t4311 * t4305;
    let t18548 = 8.0 * t18547;
    let t18549 = 0.21687162600603479684e-1 * t14363;
    let t18550 = t18298 * t162;
    (t18535, t18536, t18537, t18538, t18541, t18543, t18546, t18548, t18549, t18550)
}
