//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1276/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1276<F: Float>(t2706: F, t135: F, t144: F, t1535: F, t1536: F, t1676: F, t16825: F, t16946: F, t16950: F, t24194: F, t24670: F, t24672: F, t24674: F, t24675: F, t25058: F, t2536: F, t2718: F, t3396: F, t3401: F, t5082: F, t5191: F, t7197: F) -> (F,) {
    let t25065 = t2706 * t2706;
    let t25070 = -2.0 * t135 * t144 * t1676 * t25065 + 3.0 * t1535 * t3396 * t5191 + 24.0 * t1536 * t24194 * t2718 + 8.0 * t25058 * t2536 * t7197 - 6.0 * t2718 * t3401 * t5082 + t16825 + t16946 + t16950 - t24670 + t24672 - t24674 + t24675;
    (t25070,)
}
