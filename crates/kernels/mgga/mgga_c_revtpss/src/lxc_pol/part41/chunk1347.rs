//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1347/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1347<F: Float>(t116912: F, t31538: F, t105880: F, t116946: F, t117450: F, t117457: F, t117460: F, t117462: F, t117470: F, t117473: F, t117482: F, t117484: F, t117497: F, t117510: F, t21850: F, t21876: F, t31035: F, t31039: F, t31058: F, t5823: F, t5891: F, t5895: F, t5915: F, t658: F, t665: F, t8258: F, t8259: F, t8267: F, t8268: F) -> (F,) {
    let t118348 = t116912 * t31538;
    let t118353 = -t117450 - 110.0 / 27.0 * t117457 - t117460 + 10.0 / 9.0 * t117462 + 44.0 / 9.0 * t117470 + t117473 - t117482 + t117484 + t117497 - 3.0 / 4.0 * t31035 * t8259 * t105880 + 5.0 / 12.0 * t8258 * t8268 * t5915 * t658 + 5.0 / 18.0 * t8258 * t31058 * t5895 * t665 + 5.0 / 108.0 * t8267 * t116946 * t5895 * t658 + 5.0 / 12.0 * t8258 * t8268 * t5823 * t665 - 5.0 / 36.0 * t8267 * t31058 * t5823 * t658 - t117510 - 5.0 / 24.0 * t8267 * t8268 * t21850 + t8258 * t8259 * t21876 / 4.0 + 2.0 * t118348 + 5.0 / 4.0 * t31035 * t31039 * t5891;
    (t118353,)
}
