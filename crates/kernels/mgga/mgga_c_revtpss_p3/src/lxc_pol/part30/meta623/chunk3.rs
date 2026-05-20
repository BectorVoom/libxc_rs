//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2146/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2146<F: Float>(t98972: F, t25240: F, t2710: F, t4371: F, t10744: F, t4353: F, t7028: F, t92971: F, t92976: F, t92979: F, t98960: F, t98961: F, t98962: F, t98964: F, t98968: F, t98970: F) -> F {
    let t98973 = F::cast_from(0.2032800112371413129e-3_f64) * t98972;
    let t98976 = t2710 * t25240 * t4371;
    let t98979 = t10744 * t7028 * t4353;
    let t98981 = t98960 - t98961 - t98962 + F::new(7.0) / F::new(144.0) * t92971 - F::cast_from(0.15244095330869239812e-3_f64) * t98964 - F::cast_from(0.57165357490759649296e-3_f64) * t98968 - F::cast_from(0.17149607247227894789e-2_f64) * t98970 - t98973 + t92976 - F::new(7.0) / F::new(48.0) * t92979 - F::cast_from(0.36143185997963725434e-4_f64) * t98976 + F::cast_from(0.50820002809285328225e-5_f64) * t98979;
    t98981
}
