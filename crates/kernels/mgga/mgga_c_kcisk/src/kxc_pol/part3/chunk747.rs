//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 747/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk747<F: Float>(t5111: F, t960: F, t10442: F, t1835: F, t5114: F, t965: F, t11513: F, t11516: F, t11519: F, t11524: F, t11528: F, t11532: F, t11533: F, t11535: F, t158: F, t165: F, t173: F) -> F {
    let t11537 = t960 * t5111;
    let t11539 = t1835 * t10442;
    let t11542 = t965 * t5114;
    let t11544 = -F::new(0.4755e-2) * t165 * t11513 - F::new(0.30247875e-4) * t173 * t11516 - F::new(0.1585e-2) * t165 * t11519 - t11524 + t11528 + t11532 - F::new(0.32788e-1) * t11533 + F::new(0.10566666666666666666e-1) * t11535 - F::new(0.28104e-1) * t11537 - F::new(0.21078e-1) * t158 * t11539 + F::new(0.79249999999999999999e-2) * t11542;
    t11544
}
