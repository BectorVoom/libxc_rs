//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1113/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1113<F: Float>(t4680: F, t7426: F, t8605: F, t30468: F, t4916: F, t31346: F, t4419: F, t15386: F, t31195: F, t35749: F, t17912: F, t2288: F, t31443: F, t3169: F) -> (F, F, F, F, F) {
    let t35797 = t7426 * t4680 * t8605;
    let t35799 = t30468 * t4916;
    let t35801 = t31346 * t4419;
    let t35804 = t31195 * t15386 * t35749;
    let t35808 = t31443 * t17912 * t2288 * t3169;
    (t35797, t35799, t35801, t35804, t35808)
}
