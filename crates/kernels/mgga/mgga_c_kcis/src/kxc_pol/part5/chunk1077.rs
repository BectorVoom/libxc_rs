//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1077/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1077<F: Float>(t169: F, t18443: F, t234: F, t441: F, t233: F, t1641: F, t6888: F, t6295: F, t911: F, t6883: F, t915: F, t1881: F, t6261: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t18444 = piecewise3::<f64>(t170, F::new(0.0), t18443);
    let t18445 = t234 * t18444;
    let t18446 = t18445 * t441;
    let t18447 = t233 * t18446;
    let t18449 = t6888 * t1641;
    let t18451 = t911 * t6295;
    let t18453 = t915 * t6883;
    let t18454 = t233 * t18453;
    let t18456 = t1881 * t6261;
    (t18447, t18449, t18451, t18454, t18456)
}
