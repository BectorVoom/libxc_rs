//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 956/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk956<F: Float>(t1980: F, t33901: F, t7458: F, t30091: F, t30090: F, t8952: F, t30123: F, t30151: F, t30217: F, t2297: F, t4210: F, t13364: F, t31115: F) -> (F, F, F, F, F, F, F, F) {
    let t33903 = t1980 * t7458 * t33901;
    let t33908 = F::cast_from(0.42874018118069736972e-3_f64) * t30091;
    let t33916 = t30090 * t8952;
    let t33922 = F::cast_from(0.85748036236139473944e-3_f64) * t30123;
    let t33927 = F::cast_from(0.12579236915841660827e-2_f64) * t30151;
    let t33936 = F::cast_from(0.27953859812981468505e-2_f64) * t30217;
    let t33938 = t2297 * t4210;
    let t33940 = t31115 * t13364 * t33938;
    (t33903, t33908, t33916, t33922, t33927, t33936, t33938, t33940)
}
