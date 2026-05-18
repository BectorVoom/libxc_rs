//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 799/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk799<F: Float>(t1109: F, t2844: F, t1114: F, t3255: F, t4576: F, t4582: F, t4568: F, t10386: F, t347: F, t1022: F, t3201: F, t1714: F, t9562: F) -> (F, F, F, F, F, F, F, F) {
    let t14322 = t1109 * t2844;
    let t14326 = t1114 * t2844;
    let t14339 = F::new(0.8760572888888888889e-3) * t3255 * t4576;
    let t14341 = F::new(0.17521145777777777778e-2) * t3255 * t4582;
    let t14343 = F::new(0.14600954814814814815e-2) * t3255 * t4568;
    let t14347 = t10386 * t347;
    let t14381 = t3201 * t1022;
    let t14390 = t9562 * t1714;
    (t14322, t14326, t14339, t14341, t14343, t14347, t14381, t14390)
}
