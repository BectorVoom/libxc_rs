//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2994/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2994<F: Float>(t14322: F, t2516: F, t2496: F, t14426: F, t177: F, t762: F, t10428: F, t4305: F, t2609: F, t4186: F, t706: F, t10436: F, t4311: F) -> (F, F, F, F, F, F) {
    let t49957 = t14322 * t2516;
    let t49963 = t14322 * t2496;
    let t49966 = t14426 * t177 * t762;
    let t49978 = t10428 * t4305;
    let t49981 = t706 * t2609 * t4186;
    let t49983 = t4311 * t10436;
    (t49957, t49963, t49966, t49978, t49981, t49983)
}
