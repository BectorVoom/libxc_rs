//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2383/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2383<F: Float>(t10732: F, t10744: F, t808: F, t10674: F, t2710: F, t2713: F, t2693: F, t9732: F, t14917: F, t2475: F, t2661: F, t2662: F, t836: F) -> (F, F, F, F) {
    let t40529 = t10744 * t808 * t10732;
    let t40532 = t2710 * t2713 * t10674;
    let t40535 = t2710 * t9732 * t2693;
    let t40549 = t2661 * t2662 * t2475 * t836 * t14917;
    (t40529, t40532, t40535, t40549)
}
