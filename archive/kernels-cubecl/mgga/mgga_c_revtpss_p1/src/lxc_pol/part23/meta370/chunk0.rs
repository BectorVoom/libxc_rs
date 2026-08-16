//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1699/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1699<F: Float>(t3211: F, t4845: F, t1053: F, t4857: F, t1663: F, t371: F, t676: F, t1025: F, t11922: F, t4901: F, t4899: F, t12116: F, t4891: F) -> (F, F, F, F, F, F, F) {
    let t15744 = F::cast_from(0.15244095330869239812e-2_f64) * t3211 * t4845;
    let t15745 = t4857 * t1053;
    let t15749 = t371 * t676 * t1663;
    let t15750 = t1025 * t15749;
    let t15752 = t11922 * t4901;
    let t15754 = F::cast_from(0.28582678745379824648e-3_f64) * t4899 * t15752;
    let t15758 = t12116 * t4891;
    (t15744, t15745, t15749, t15750, t15752, t15754, t15758)
}
