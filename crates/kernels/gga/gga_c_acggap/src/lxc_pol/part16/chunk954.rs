//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 954/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk954<F: Float>(t3201: F, t8489: F, t1980: F, t7458: F, t30090: F, t8952: F, t2297: F, t4210: F, t13364: F, t31115: F, t1: F, t1170: F, t2065: F, t8461: F) -> (F, F, F, F, F, F) {
    let t33901 = t3201 * t8489;
    let t33903 = t1980 * t7458 * t33901;
    let t33904 = F::new(0.28582678745379824648e-3) * t33903;
    let t33916 = t30090 * t8952;
    let t33938 = t2297 * t4210;
    let t33940 = t31115 * t13364 * t33938;
    let t33941 = F::new(0.10718504529517434243e-2) * t33940;
    let t33944 = t1170 * t2065 * t8461 * t1;
    (t33901, t33904, t33916, t33938, t33941, t33944)
}
