//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 891/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk891<F: Float>(t270: F, t3030: F, t9725: F, t9728: F, t999: F, t292: F, t737: F, t285: F, t1071: F, t240: F, t9: F, t2866: F, t990: F, t2872: F, t2881: F, t2880: F, t2900: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9825 = 1.0 / t3030 / t270;
    let t9851 = 0.93932222222222222223e0 * t9725;
    let t9852 = 0.36793333333333333333e0 * t9728;
    let t9873 = t999 * t999;
    let t9874 = 1.0 / t9873;
    let t9881 = t737 * t292;
    let t9883 = 5.0 / 1296.0 * t285 * t9881;
    let t9896 = 1.0 / t240 / t1071;
    let t9897 = t9 * t9896;
    let t9903 = t2866 * t990;
    let t9906 = t2872 * t2881;
    let t9909 = t2880 * t2900;
    (t9825, t9851, t9852, t9874, t9883, t9897, t9903, t9906, t9909)
}
