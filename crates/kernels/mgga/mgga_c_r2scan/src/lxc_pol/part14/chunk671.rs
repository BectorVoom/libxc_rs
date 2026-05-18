//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 671/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk671<F: Float>(t48: F, t4902: F, t35: F, t40: F, t51: F, t53: F, t139: F, t141: F, t1524: F, t378: F, t735: F, t1385: F) -> (F, F, F, F, F, F, F) {
    let t4904 = F::new(1.0) / t48 / t4902;
    let t4911 = t35 * t40;
    let t4918 = t51 * t51;
    let t4920 = F::new(1.0) / t53 / t4918;
    let t4938 = F::new(1.0) / t139;
    let t4948 = F::new(1.0) / t141;
    let t4962 = t378 * t1524;
    let t4963 = t735 * t4962;
    let t4964 = F::new(0.32530743900905219526e-1) * t4963;
    let t4965 = t378 * t1385;
    (t4904, t4911, t4920, t4938, t4948, t4964, t4965)
}
