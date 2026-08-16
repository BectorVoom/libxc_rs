//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3144/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3144<F: Float>(t1276: F, t5245: F, t460: F, t488: F, t13181: F, t1828: F, t12627: F, t12626: F, t1769: F, t487: F, t1770: F, t3727: F) -> (F, F, F, F, F, F, F) {
    let t56310 = t1276 * t5245;
    let t56314 = t460 * t488;
    let t56315 = t13181 * t1828;
    let t56327 = t12627 * t488;
    let t56331 = t1769 * t12626;
    let t56332 = t56331 * t487;
    let t56384 = t1770 * t3727;
    (t56310, t56314, t56315, t56327, t56331, t56332, t56384)
}
