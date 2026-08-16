//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3309/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3309<F: Float>(t1448: F, t6816: F, t22852: F, t4140: F, t47076: F, t48291: F, t48293: F, t5536: F, t85923: F, t85924: F, t85925: F, t85926: F, t85928: F, t85930: F, t85932: F) -> (F, F) {
    let t86771 = t6816 * t1448;
    let t86782 = F::cast_from(18.0_f64) * t22852 * t4140 * t5536 - t47076 + t48291 - t48293 - t85923 + t85924 - t85925 + t85926 - t85928 + t85930 - t85932;
    (t86771, t86782)
}
