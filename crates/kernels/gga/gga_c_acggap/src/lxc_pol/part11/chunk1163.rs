//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1163/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1163<F: Float>(t35934: F, t31276: F, t8544: F, t7310: F, t8505: F, t2001: F, t4894: F, t4878: F, t30225: F, t542: F, t1588: F, t7605: F) -> (F, F, F, F, F, F, F) {
    let t35935 = F::new(0.21437009059034868486e-3) * t35934;
    let t35936 = t31276 * t8544;
    let t35938 = t7310 * t8505;
    let t35942 = t2001 * t4894;
    let t35947 = t2001 * t4878;
    let t35949 = t30225 * t542;
    let t35951 = t7605 * t1588;
    (t35935, t35936, t35938, t35942, t35947, t35949, t35951)
}
