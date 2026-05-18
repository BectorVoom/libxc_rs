//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 841/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk841<F: Float>(t4999: F, t5020: F, t1627: F, t2685: F, t1764: F, t995: F, t1403: F, t1821: F, t1820: F, t5002: F, t954: F, t1413: F) -> (F, F, F, F, F) {
    let t7083 = F::new(16.0) / F::new(135.0) * t4999;
    let t7084 = F::new(16.0) / F::new(45.0) * t5020;
    let t7086 = F::new(8.0) / F::new(45.0) * t1627 * t2685;
    let t7087 = t995 * t1764;
    let t7088 = t7087 * t1403;
    let t7089 = t1821 * t7088;
    let t7091 = F::new(16.0) / F::new(45.0) * t1820 * t7089;
    let t7092 = t5002 * t954;
    let t7093 = t7092 * t1413;
    (t7083, t7084, t7086, t7091, t7093)
}
