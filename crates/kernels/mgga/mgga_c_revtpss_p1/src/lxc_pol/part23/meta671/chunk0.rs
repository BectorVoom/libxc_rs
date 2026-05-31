//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2406/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2406<F: Float>(t271: F, t2852: F, t1054: F, t11970: F, t11986: F, t828: F, t11631: F, t905: F, t606: F, t1086: F, t11223: F, t3090: F) -> (F, F, F, F, F) {
    let t43222 = F::cast_from(1.0_f64) / t271 / t2852;
    let t43238 = t1054 * t11970;
    let t43240 = t828 * t11986;
    let t43253 = t11631 * t905;
    let t43254 = t43253 * t606;
    let t43285 = t11223 * t1086 * t3090;
    (t43222, t43238, t43240, t43254, t43285)
}
