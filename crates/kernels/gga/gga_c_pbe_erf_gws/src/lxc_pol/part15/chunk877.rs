//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 877/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk877<F: Float>(t43: F, t8291: F, t8323: F, t8500: F, t8514: F, t312: F, t1: F, t1098: F, t2057: F, t2062: F, t1167: F, t6854: F, t6868: F, t810: F, t8079: F, t8082: F, t8084: F, t8086: F, t8088: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t8516 = t8291 + t8323 + t8500 + t8514;
    let t8517 = t8516 * t312;
    let t8519 = t1098 * t2057 * t1;
    let t8520 = t8519 * t2062;
    let t8521 = 0.63272429661648472106e0 * t8520;
    let t8546 = t1167 * t6854;
    let t8556 = t6868 * t810;
    let t8565 = piecewise3(t44, 0.0, 8.0 / 27.0 * t8079 - 8.0 / 9.0 * t8082 - 2.0 / 9.0 * t8084 + 4.0 / 3.0 * t8086 - 4.0 * t8088);
    (t8517, t8521, t8546, t8556, t8565)
}
