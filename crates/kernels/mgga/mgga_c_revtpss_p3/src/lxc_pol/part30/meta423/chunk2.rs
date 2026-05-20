//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1601/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1601<F: Float>(t4186: F, t999: F, t4872: F, t1042: F, t4866: F, t73: F, t3095: F, t3092: F, t2857: F, t357: F, t2251: F, t4781: F) -> (F, F, F, F) {
    let t15950 = t4186 * t999;
    let t15951 = t4872 * t15950;
    let t15952 = t1042 * t15951;
    let t15957 = t4866 * t73;
    let t15958 = t15957 * t3095;
    let t15959 = t3092 * t15958;
    let t15962 = t357 * t2857;
    let t15963 = t15962 * t2251;
    let t15964 = t4781 * t15963;
    (t15952, t15957, t15959, t15964)
}
