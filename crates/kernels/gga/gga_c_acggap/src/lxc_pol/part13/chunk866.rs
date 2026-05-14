//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 866/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk866<F: Float>(t2131: F, t2132: F, t309: F, t7877: F, t7980: F, t7987: F, t1264: F, t2138: F, t2139: F, t2147: F, t463: F, t7993: F, t29997: F, t4210: F, t7942: F, t7976: F, t880: F) -> (F, F, F, F, F, F) {
    let t31895 = t2131 * t2132 * t7877 * t309;
    let t31897 = t7987 * t7980;
    let t31901 = t2138 * t2147 * t2139 * t1264;
    let t31905 = t2138 * t2147 * t7993 * t463;
    let t31912 = t7942 * t29997 * t4210;
    let t31916 = t7976 * t880;
    (t31895, t31897, t31901, t31905, t31912, t31916)
}
