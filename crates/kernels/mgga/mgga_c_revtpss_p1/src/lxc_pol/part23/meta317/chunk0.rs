//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1606/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1606<F: Float>(t1514: F, t2289: F, t4264: F, t625: F, t4288: F, t2339: F, t4287: F, t2349: F, t97: F, t105: F, t2357: F, t1468: F, t9335: F) -> (F, F, F, F, F, F, F) {
    let t13448 = t2289 * t1514;
    let t13451 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t625 * t4264;
    let t13453 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t625 * t4288;
    let t13458 = t2339 * t4287;
    let t13475 = t97 * t2349;
    let t13496 = t105 * t2357;
    let t13550 = t9335 * t1468;
    (t13448, t13451, t13453, t13458, t13475, t13496, t13550)
}
