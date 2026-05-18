//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1137/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1137<F: Float>(t10556: F, t1775: F, t10566: F, t10614: F, t2772: F, t8282: F, t10600: F, t10604: F, t665: F, t7640: F, t2: F, t2344: F, t2680: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43888 = t1775 * t10556;
    let t43890 = t1775 * t10566;
    let t43904 = t1775 * t10614;
    let t43906 = t8282 * t2772;
    let t43908 = t1775 * t10600;
    let t43910 = t1775 * t10604;
    let t43912 = t665 * t7640;
    let t43913 = t43912 * t2;
    let t43917 = t2344 * t2680;
    (t43888, t43890, t43904, t43906, t43908, t43910, t43912, t43913, t43917)
}
