//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 817/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk817<F: Float>(t26948: F, t7635: F, t13181: F, t473: F, t2142: F, t3566: F, t26936: F, t7642: F, t1209: F, t7627: F, t460: F, t3555: F) -> (F, F, F, F, F, F, F, F) {
    let t26949 = t26948 * t7635;
    let t26969 = t13181 * t473;
    let t26976 = t3566 * t2142;
    let t26979 = t7642 * t26936;
    let t26994 = t3566 * t7635;
    let t26999 = t1209 * t7627;
    let t27008 = t460 * t7627;
    let t27011 = t3555 * t2142;
    (t26949, t26969, t26976, t26979, t26994, t26999, t27008, t27011)
}
