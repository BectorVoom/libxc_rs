//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1515/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1515<F: Float>(t3046: F, t3316: F, t4891: F, t11923: F, t11933: F, t41229: F, t41241: F, t41243: F, t41449: F, t41451: F, t41453: F, t41455: F, t41459: F, t41468: F, t41472: F, t41476: F) -> (F, F, F) {
    let t42830 = t3046 * t3316 * t4891;
    let t42833 = t11933 * t11923;
    let t42846 = t41229 - t41241 - t41243 - t41449 + t41451 - t41453 - t41455 + t41459 + t41468 - t41472 - t41476;
    (t42830, t42833, t42846)
}
