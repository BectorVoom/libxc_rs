//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1112/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1112<F: Float>(t13700: F, t13714: F, t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F) -> (F, F, F, F) {
    let t13716 = t13700 / F::new(2.0) + t13714 / F::new(2.0);
    let t13725 = t785 * t1892;
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    let t13729 = t4075 * t1903;
    let t13730 = t13729 * t1444;
    let t13731 = t556 * t13730;
    let t13733 = F::new(0.21951497276451705328e-1) * t2782 * t13731;
    let t13734 = t212 * t5710;
    (t13716, t13727, t13733, t13734)
}
