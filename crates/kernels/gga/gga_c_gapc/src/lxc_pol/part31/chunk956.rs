//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 956/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk956<F: Float>(t7158: F, t7591: F, t8134: F, t7877: F, t875: F, t103: F, t15512: F, t966: F, t15513: F, t786: F, t1180: F, t15805: F, t2206: F, t2394: F, t2211: F, t2254: F) -> (F, F, F, F, F, F, F) {
    let t18018 = t7591 * t7158 * t8134;
    let t18105 = t875 * t7877;
    let t18107 = t15512 * t966 * t18105 * t103;
    let t18317 = t15513 * t786 * t7877 * t103;
    let t18331 = t15805 * t1180;
    let t18551 = t2394 * t2206;
    let t18553 = t2211 * t2254;
    (t18018, t18105, t18107, t18317, t18331, t18551, t18553)
}
