//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1138/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1138<F: Float>(t27940: F, t9842: F, t26028: F, t9832: F, t9828: F, t25983: F, t9914: F, t2482: F, t596: F, t7269: F, t3981: F, t7271: F, t9944: F) -> (F, F, F, F, F, F) {
    let t94434 = t27940 * t9842;
    let t94436 = t26028 * t9832;
    let t94438 = t26028 * t9828;
    let t94440 = t25983 * t9914;
    let t94443 = t2482 * t7269 * t596;
    let t94444 = t94443 * t3981;
    let t94446 = t7271 * t9944;
    (t94434, t94436, t94438, t94440, t94444, t94446)
}
