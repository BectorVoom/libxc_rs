//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 773/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk773<F: Float>(t3255: F, t5465: F, t11633: F, t1897: F, t518: F, t5481: F, t10338: F, t1988: F, t1890: F, t2323: F) -> (F, F, F, F, F) {
    let t16003 = 0.19711289e-2 * t3255 * t5465;
    let t16025 = t11633 * t1897;
    let t16029 = t518 * t5481;
    let t16038 = t10338 * t1988;
    let t16046 = t2323 * t1890;
    (t16003, t16025, t16029, t16038, t16046)
}
