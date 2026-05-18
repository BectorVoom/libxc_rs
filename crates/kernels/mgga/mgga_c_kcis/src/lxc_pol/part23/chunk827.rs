//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 827/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk827<F: Float>(t1014: F, t5891: F, t3728: F, t5629: F, t1962: F, t3841: F, t5498: F, t14249: F, t5446: F, t3781: F, t5493: F, t5457: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15986 = t1014 * t5891;
    let t15987 = F::new(0.88437037037037037034e-2) * t15986;
    let t15988 = t3728 * t5629;
    let t15989 = F::new(0.33163888888888888888e-2) * t15988;
    let t15990 = t1962 * t3841;
    let t15991 = t5498 * t15990;
    let t15994 = t14249 * t5446;
    let t15996 = t5493 * t3781;
    let t15997 = t5457 * t15996;
    (t15986, t15987, t15988, t15989, t15990, t15991, t15994, t15996, t15997)
}
