//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 803/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk803<F: Float>(t1259: F, t135: F, t2464: F, t273: F, t3736: F, t3738: F, t3742: F, t3768: F, t3771: F, t3827: F, t3829: F, t3831: F, t3835: F, t3839: F, t3843: F, t3932: F, t957: F) -> (F, F) {
    let t3936 = t1259 * t1259;
    let t3940 = -t135 * t2464 * t273 * t3936 + t135 * t273 * t3932 * t957 - t3736 + t3738 - t3742 + t3768 + t3771 + t3827 + t3829 - t3831 + t3835 - t3839 - t3843;
    (t3936, t3940)
}
