//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2257/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2257<F: Float>(t60224: F, t6957: F, t1493: F, t2315: F, t77: F, t2259: F, t4173: F, t38: F, t60248: F, t1928: F, t25114: F, t25120: F, t25140: F, t25143: F, t25159: F, t28093: F, t28127: F, t28138: F, t6958: F, t6974: F, t6978: F, t7702: F, t7716: F, t7720: F) -> F {
    let t101342 = t60224 * t6957;
    let t101350 = t77 * t1493 * t2315;
    let t101357 = t4173 * t2259;
    let t101360 = t60248 * t38;
    let t101371 = -F::cast_from(5.0_f64) * t101342 * t25159 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t28127 * t25114 + t25120 * t7716 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6958 * t101350 + t25120 * t7720 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t28138 * t25114 + t101357 * t1928 / F::cast_from(3.0_f64) - t101360 * t1928 / F::cast_from(6.0_f64) - t28093 * t6974 / F::cast_from(3.0_f64) - t28093 * t6978 / F::cast_from(3.0_f64) - t7702 * t25140 / F::cast_from(6.0_f64) - t7702 * t25143 / F::cast_from(3.0_f64);
    t101371
}
