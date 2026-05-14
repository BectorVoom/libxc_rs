//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 795/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk795<F: Float>(t1396: F, t6912: F, t1395: F, t1394: F, t2001: F, t2011: F) -> (F, F, F, F) {
    let t6913 = t1396 * t6912;
    let t6914 = t1395 * t6913;
    let t6915 = t1394 * t6914;
    let t6917 = t2001 * t2011;
    (t6913, t6914, t6915, t6917)
}
