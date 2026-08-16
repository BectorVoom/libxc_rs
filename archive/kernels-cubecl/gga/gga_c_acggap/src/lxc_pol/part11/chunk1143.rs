//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1143/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1143<F: Float>(t35685: F, t13299: F, t33944: F, t33945: F, t13287: F, t2297: F, t31195: F, t3169: F, t15386: F, t35340: F, t2288: F, t4210: F) -> (F, F, F, F, F) {
    let t35686 = F::cast_from(11.0_f64) / F::cast_from(48.0_f64) * t35685;
    let t35691 = t33944 * t13299 * t33945;
    let t35695 = t31195 * t13287 * t2297 * t3169;
    let t35698 = t31195 * t15386 * t35340;
    let t35700 = t2288 * t4210;
    (t35686, t35691, t35695, t35698, t35700)
}
