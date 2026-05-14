//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 801/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk801<F: Float>(t1036: F, t5463: F, t639: F, t1802: F, t1804: F, t995: F, t1885: F, t1820: F, t188: F, t331: F, t34: F, t597: F, t610: F, t1033: F, t1683: F, t2816: F, t663: F) -> (F, F, F, F, F) {
    let t7459 = t5463 * t1036;
    let t7460 = t639 * t7459;
    let t7461 = 8.0 / 405.0 * t7460;
    let t7463 = t1802 * t995 * t1804;
    let t7464 = t1885 * t7463;
    let t7466 = 8.0 / 15.0 * t1820 * t7464;
    let t7467 = t331 * t188;
    let t7468 = t597 * t34;
    let t7469 = t7468 * t610;
    let t7470 = t7467 * t7469;
    let t7472 = 8.0 / 15.0 * t1820 * t7470;
    let t7474 = 8.0 / 45.0 * t1033 * t1683;
    let t7476 = 4.0 / 15.0 * t2816 * t663;
    (t7461, t7466, t7472, t7474, t7476)
}
