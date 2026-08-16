//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 995/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk995<F: Float>(t1873: F, t9741: F, t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F, t2482: F, t4000: F, t814: F, t136: F, t550: F) -> (F, F, F, F, F, F) {
    let t13798 = t9741 * t1873;
    let t13800 = t808 * t5651;
    let t13801 = t9736 * t13800;
    let t13804 = t820 * t9991 * t241;
    let t13845 = t2482 * t4000 * t814;
    let t13846 = t550 * t136;
    (t13798, t13800, t13801, t13804, t13845, t13846)
}
