//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1077/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1077<F: Float>(t1163: F, t21631: F, t1312: F, t3583: F, t6500: F, t14943: F, t14945: F, t14948: F, t14957: F, t15011: F, t1580: F, t21608: F, t21612: F, t21617: F, t21621: F, t21626: F, t2322: F) -> (F,) {
    let t21632 = t21631 * t1163;
    let t21633 = t1312 * t21632;
    let t21636 = t6500 * t3583;
    let t21637 = t1312 * t21636;
    let t21640 = -0.71963154864709268852e-1 * t1580 * t21608 - 0.71963154864709268855e-1 * t1580 * t21612 - 0.16191709844559585492e0 * t1580 * t21617 - 0.19989765240197019126e-2 * t21621 + 0.87954967056866884154e-1 * t15011 * t2322 - t21626 + 0.11993859144118211476e-1 * t14943 - 0.17990788716177317213e-1 * t14945 - 0.89953943580886586067e-2 * t14948 + 0.17990788716177317213e-1 * t14957 - 0.17990788716177317213e-1 * t1580 * t21633 - 0.89953943580886586067e-2 * t1580 * t21637;
    (t21640,)
}
