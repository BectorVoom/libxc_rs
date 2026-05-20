//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2186/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2186<F: Float>(t100882: F, t100926: F, t18875: F, t94245: F, t25759: F, t61203: F, t98674: F, t98759: F, t98651: F, t15071: F, t33: F, t1940: F, t2403: F, t25206: F, t25781: F, t27158: F, t27364: F, t27368: F, t27764: F, t3351: F, t7091: F, t7200: F, t7783: F, t98635: F, t98650: F, t98669: F, t98684: F, t99537: F) -> (F, F) {
    let t100927 = t100882 + t100926;
    let t100944 = t94245 * t18875;
    let t100947 = t25759 * t61203;
    let t100953 = t25759 * t98674;
    let t100958 = t25759 * t98759;
    let t100964 = t25759 * t98651;
    let t100969 = t33 * t15071;
    let t100973 = t98635 - t98650 + t1940 * t99537 * t33 / F::new(2.0) - F::new(3.0) * t25206 * t100944 - t98684 - F::new(3.0) / F::new(2.0) * t25206 * t100947 + t1940 * t7783 * t3351 / F::new(2.0) - F::new(6.0) * t27158 * t100953 + F::new(6.0) * t98669 * t27764 - F::new(3.0) * t27158 * t100958 + F::new(3.0) * t2403 * t27364 * t7200 - F::new(3.0) / F::new(2.0) * t25206 * t100964 - t1940 * t27368 * t25781 - t1940 * t7091 * t100969 / F::new(2.0);
    (t100927, t100973)
}
