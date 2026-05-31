//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1488/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1488<F: Float>(t116912: F, t31538: F, t105880: F, t116946: F, t117450: F, t117457: F, t117460: F, t117462: F, t117470: F, t117473: F, t117482: F, t117484: F, t117497: F, t117510: F, t21850: F, t21876: F, t31035: F, t31039: F, t31058: F, t5823: F, t5891: F, t5895: F, t5915: F, t658: F, t665: F, t8258: F, t8259: F, t8267: F, t8268: F) -> F {
    let t118348 = t116912 * t31538;
    let t118353 = -t117450 - F::cast_from(110.0_f64) / F::cast_from(27.0_f64) * t117457 - t117460 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t117462 + F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t117470 + t117473 - t117482 + t117484 + t117497 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t31035 * t8259 * t105880 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8258 * t8268 * t5915 * t658 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t8258 * t31058 * t5895 * t665 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t8267 * t116946 * t5895 * t658 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8258 * t8268 * t5823 * t665 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8267 * t31058 * t5823 * t658 - t117510 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8267 * t8268 * t21850 + t8258 * t8259 * t21876 / F::cast_from(4.0_f64) + F::cast_from(2.0_f64) * t118348 + F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t31035 * t31039 * t5891;
    t118353
}
