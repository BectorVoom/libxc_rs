//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 693/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk693<F: Float>(t1762: F, t5957: F, t1835: F, t377: F, t1946: F, t1767: F, t1987: F, t424: F, t625: F, t1764: F, t1768: F, t1693: F, t5714: F, t61: F, t1793: F, t410: F) -> (F, F, F, F, F, F, F, F) {
    let t5959 = 0.32530743900905219526e-1 * t1762 * t5957;
    let t5960 = t377 * t1835;
    let t5961 = t5960 * t1946;
    let t5963 = 0.28895839882605942646e1 * t1762 * t5961;
    let t5964 = t1767 * t1987;
    let t5966 = 0.96319466275353142157e0 * t1762 * t5964;
    let t5967 = t424 * t625;
    let t5968 = t5967 * t1764;
    let t5970 = t5967 * t1768;
    let t5972 = t424 * t1693;
    let t5975 = 0.11558335953042377058e2 * t61 * t5714;
    let t5976 = t410 * t1793;
    (t5959, t5963, t5966, t5968, t5970, t5972, t5975, t5976)
}
