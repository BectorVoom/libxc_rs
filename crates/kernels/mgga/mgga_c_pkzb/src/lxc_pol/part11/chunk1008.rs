//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1008/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1008<F: Float>(t1147: F, t3702: F, t10757: F, t10779: F, t10782: F, t10785: F, t10930: F, t10951: F, t10954: F, t10962: F, t10966: F, t10970: F, t10972: F, t11104: F, t135: F, t273: F, t6065: F, t805: F) -> (F, F) {
    let t11108 = t3702 * t1147;
    let t11113 = t11104 * t135 * t273 * t805 + F::new(2.0) * t11108 * t135 * t273 * t6065 + t10757 - t10779 - t10782 + t10785 + t10930 - t10951 - t10954 - t10962 + t10966 - t10970 - t10972;
    (t11108, t11113)
}
