//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 639/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk639<F: Float>(t15868: F, t419: F, t1527: F, t15768: F, t15625: F, t423: F, t420: F, t15847: F, t15850: F, t15852: F, t15855: F, t15858: F, t15861: F, t15863: F, t15866: F, t15845: F) -> (F, F, F, F) {
    let t15869 = t419 * t15868;
    let t15871 = t1527 * t15768;
    let t15872 = t419 * t15871;
    let t15874 = t423 * t15625;
    let t15875 = t420 * t15874;
    let t15876 = t419 * t15875;
    let t15878 = -0.51074886703703703704e-1 * t15847 + 0.34049924469135802469e-1 * t15850 + 0.34049924469135802469e-1 * t15852 - 0.42562405586419753087e-2 * t15855 + 0.38306165027777777778e-1 * t15858 - 0.51074886703703703704e-1 * t15861 - 0.17024962234567901235e-1 * t15863 + 0.21281202793209876543e-2 * t15866 + 0.85124811172839506173e-2 * t15869 - 0.12768721675925925926e-1 * t15872 + 0.6384360837962962963e-2 * t15876;
    let t15879 = t15845 + t15878;
    (t15869, t15872, t15876, t15879)
}
