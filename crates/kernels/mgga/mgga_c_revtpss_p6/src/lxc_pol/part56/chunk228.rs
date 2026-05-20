//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 228/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk228<F: Float>(t996: F, t999: F, t344: F, t614: F, t139: F, t221: F, t346: F, t345: F, t220: F, t44: F) -> (F, F, F, F, F, F) {
    let t1000 = t996 * t999;
    let t1003 = t614 * t344;
    let t1007 = t221 * t139 * t346;
    let t1009 = t345 * t1007 / F::new(288.0);
    let t1010 = t344 * t220;
    let t1011 = t44 * t1010;
    (t1000, t1003, t1007, t1009, t1010, t1011)
}
