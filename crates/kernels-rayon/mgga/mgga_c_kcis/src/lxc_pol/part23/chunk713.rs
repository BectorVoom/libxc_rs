//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 713/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk713(t1629: f64, t187: f64, t2128: f64, t2268: f64, t4480: f64, t6222: f64, t633: f64, t7998: f64, t8183: f64, t8184: f64, t8185: f64, t8188: f64, t8208: f64, t8236: f64, t8240: f64, t8251: f64) -> f64 {
    let t8255 = t8183 - t8184 - t8185 + t8188 - t8208 + t187 * (-t1629 * t8251 - t2128 * t7998 - t2268 * t6222 + 2.0_f64 * t4480 * t8240 + t633 * t8236 - t8183 + t8184 + t8185 - t8188 + t8208);
    t8255
}
