//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 825/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk825<F: Float>(t22873: F, t5508: F, t28: F, t376: F, t5509: F, t1286: F, t1307: F, t7241: F, t108: F, t1588: F, t5619: F, t497: F, t5618: F, t1308: F, t1920: F, t1564: F, t1647: F, t5502: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22874 = t22873 * t5508;
    let t22875 = t28 * t22874;
    let t22878 = t376 * t5509;
    let t22879 = t1286 * t22878;
    let t22883 = t7241 * t1307;
    let t22884 = t108 * t1588;
    let t22885 = t22883 * t22884;
    let t22886 = t28 * t22885;
    let t22892 = t376 * t5619;
    let t22893 = t1286 * t22892;
    let t22895 = t5618 * t497;
    let t22896 = t28 * t22895;
    let t22899 = t1308 * t1920;
    let t22900 = t28 * t22899;
    let t22904 = t1564 * t5502 * t1647;
    (t22874, t22875, t22878, t22879, t22883, t22884, t22885, t22886, t22892, t22893, t22895, t22896, t22899, t22900, t22904)
}
