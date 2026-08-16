//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2930/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2930<F: Float>(t3201: F, t4798: F, t343: F, t44: F, t816: F, t11821: F, t65: F, t11144: F, t11970: F, t1660: F, t27527: F, t2852: F) -> (F, F, F, F, F) {
    let t53317 = t4798 * t3201;
    let t53318 = F::cast_from(0.14291339372689912324e-3_f64) * t53317;
    let t53320 = t44 * t343 * t816;
    let t53321 = t65 * t11821;
    let t53322 = t53321 * t11144;
    let t53326 = t1660 * t11970;
    let t53328 = t27527 * t2852;
    (t53318, t53320, t53322, t53326, t53328)
}
