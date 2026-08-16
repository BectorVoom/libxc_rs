//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1107/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1107<F: Float>(t13287: F, t2297: F, t31195: F, t3169: F, t15386: F, t35340: F, t2288: F, t4210: F, t31057: F, t3176: F, t33953: F, t31443: F) -> (F, F, F, F, F, F) {
    let t35695 = t31195 * t13287 * t2297 * t3169;
    let t35698 = t31195 * t15386 * t35340;
    let t35700 = t2288 * t4210;
    let t35702 = t31057 * t15386 * t35700;
    let t35704 = t33953 * t3176;
    let t35706 = t31443 * t13287 * t35704;
    (t35695, t35698, t35700, t35702, t35704, t35706)
}
