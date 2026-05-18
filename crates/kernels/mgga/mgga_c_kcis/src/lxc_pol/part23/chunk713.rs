//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 713/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk713<F: Float>(t1629: F, t187: F, t2128: F, t2268: F, t4480: F, t6222: F, t633: F, t7998: F, t8183: F, t8184: F, t8185: F, t8188: F, t8208: F, t8236: F, t8240: F, t8251: F) -> F {
    let t8255 = t8183 - t8184 - t8185 + t8188 - t8208 + t187 * (-t1629 * t8251 - t2128 * t7998 - t2268 * t6222 + F::new(2.0) * t4480 * t8240 + t633 * t8236 - t8183 + t8184 + t8185 - t8188 + t8208);
    t8255
}
