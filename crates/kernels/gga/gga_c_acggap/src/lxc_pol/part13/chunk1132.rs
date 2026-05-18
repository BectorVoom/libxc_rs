//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1132/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1132<F: Float>(t2001: F, t4853: F, t13287: F, t31057: F, t33953: F, t5122: F, t15386: F, t35284: F, t2302: F, t4210: F, t2260: F, t7852: F) -> (F, F, F, F, F) {
    let t35545 = t2001 * t4853;
    let t35549 = t31057 * t13287 * t33953 * t5122;
    let t35550 = F::new(0.62896184579208304136e-3) * t35549;
    let t35552 = t31057 * t15386 * t35284;
    let t35553 = F::new(0.94344276868812456204e-3) * t35552;
    let t35556 = t31057 * t13287 * t2302 * t4210;
    let t35557 = F::new(0.62896184579208304136e-3) * t35556;
    let t35560 = t7852 * t2260;
    (t35545, t35550, t35553, t35557, t35560)
}
