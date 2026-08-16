//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 962/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk962<F: Float>(t2138: F, t2147: F, t463: F, t7993: F, t29997: F, t4210: F, t7942: F, t7976: F, t880: F, t1265: F, t7973: F, t7983: F) -> (F, F, F, F, F) {
    let t31905 = t2138 * t2147 * t7993 * t463;
    let t31912 = t7942 * t29997 * t4210;
    let t31916 = t7976 * t880;
    let t31918 = t7973 * t1265;
    let t31922 = t2138 * t2147 * t7983 * t463;
    (t31905, t31912, t31916, t31918, t31922)
}
