//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2252/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2252<F: Float>(t1497: F, t6977: F, t1926: F, t1927: F, t4241: F, t25163: F, t7715: F, t101187: F, t101190: F, t101193: F, t101200: F, t101204: F, t101211: F, t10309: F, t1928: F, t25157: F, t25162: F, t28147: F, t28151: F, t32592: F, t92565: F, t92588: F) -> F {
    let t101214 = t6977 * t1497;
    let t101215 = t1926 * t101214;
    let t101218 = t1927 * t4241;
    let t101219 = t1926 * t101218;
    let t101222 = t7715 * t25163;
    let t101225 = t101187 * t1928 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101190 * t1928 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t101193 * t1928 + F::cast_from(20.0_f64) * t10309 * t32592 * t28147 - F::cast_from(10.0_f64) * t25157 * t101200 - F::cast_from(5.0_f64) * t25157 * t101204 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t92565 * t28151 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t92588 * t28151 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t101211 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t101215 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t101219 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t25162 * t101222;
    t101225
}
