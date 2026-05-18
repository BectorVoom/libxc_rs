//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1177/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1177<F: Float>(t1466: F, t36012: F, t681: F, t1212: F, t142935: F, t143293: F, t1477: F, t152972: F, t193: F, t29006: F, t29008: F, t29410: F, t33808: F, t33998: F, t34058: F, t35795: F, t36049: F, t36097: F, t6210: F, t6222: F, t6223: F, t6391: F, t6963: F, t6970: F, t7129: F, t824: F) -> F {
    let t154960 = t1466 * t681 * t36012;
    let t154983 = t6210 * t36097 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1466 * t193 * t6222 * t7129 * t824 - t6210 * t35795 / F::new(3.0) - t1466 * t193 * t143293 * t6970 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t154960 - t142935 / F::new(18.0) - F::new(2.0) / F::new(3.0) * t1466 * t193 * t6222 * t6391 * t1212 + t1466 * t193 * t1477 * t29410 / F::new(3.0) - t29008 * t33998 / F::new(9.0) + t33808 * t29006 - t1466 * t193 * t152972 * t6223 / F::new(3.0) + t6210 * t36049 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t6963 * t34058;
    t154983
}
