//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 861/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk861<F: Float>(t1980: F, t33884: F, t7458: F, t535: F, t7457: F, t7459: F, t3201: F, t8489: F, t30091: F, t30090: F, t8952: F, t30123: F, t30151: F, t30217: F, t2297: F, t4210: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33886 = t1980 * t7458 * t33884;
    let t33894 = t7457 * t7458 * t535 * t7459;
    let t33901 = t3201 * t8489;
    let t33903 = t1980 * t7458 * t33901;
    let t33908 = 0.42874018118069736972e-3 * t30091;
    let t33916 = t30090 * t8952;
    let t33922 = 0.85748036236139473944e-3 * t30123;
    let t33927 = 0.12579236915841660827e-2 * t30151;
    let t33936 = 0.27953859812981468505e-2 * t30217;
    let t33938 = t2297 * t4210;
    (t33886, t33894, t33901, t33903, t33908, t33916, t33922, t33927, t33936, t33938)
}
