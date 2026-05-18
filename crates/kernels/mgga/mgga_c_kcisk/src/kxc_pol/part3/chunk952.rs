//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 952/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk952<F: Float>(t1390: F, t213: F, t1056: F, t3283: F, t3859: F, t1387: F, t12830: F, t12924: F, t1349: F, t1391: F, t14083: F, t14084: F, t14085: F, t14088: F, t14091: F, t14093: F, t14096: F) -> F {
    let t14100 = t213 * t1390;
    let t14101 = t14100 * t1056;
    let t14103 = t3859 * t3283;
    let t14107 = t1387 * t1056;
    let t14109 = -t14083 + t14084 - F::new(0.62154466893555682512e-3) * t14085 * t12830 + F::new(0.71734315950379065738e-1) * t14088 - F::new(0.93231700340333523768e-3) * t14091 + F::new(0.71734315950379065738e-1) * t14093 * t12830 - F::new(0.35867157975189532869e-1) * t14096 + F::new(0.11955719325063177623e-1) * t1349 * t12924 - F::new(0.93231700340333523768e-3) * t14101 + F::new(0.31077233446777841256e-3) * t14103 - F::new(0.5179538907796306876e-4) * t1391 * t12924 + F::new(0.71734315950379065738e-1) * t14107;
    t14109
}
