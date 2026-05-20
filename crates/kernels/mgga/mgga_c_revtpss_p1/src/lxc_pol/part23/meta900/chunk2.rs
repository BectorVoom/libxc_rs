//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2863/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2863<F: Float>(t10696: F, t14643: F, t14648: F, t14652: F, t1553: F, t18392: F, t18435: F, t18599: F, t18612: F, t227: F, t23114: F, t23148: F, t23235: F, t23238: F, t23241: F, t4343: F, t4415: F, t4416: F, t5962: F, t76421: F, t775: F, t830: F, t832: F, t853: F) -> F {
    let t77118 = -F::new(360.0) * t10696 * t23114 * t4415 * t775 - F::new(12.0) * t23148 * t4415 * t775 * t853 + F::new(180.0) * t14648 * t18435 * t4415 - F::new(36.0) * t14652 * t4415 * t5962 - F::new(36.0) * t18392 * t4415 * t4416 + F::new(180.0) * t18599 * t4343 * t4415 + F::new(3.0) * t227 * t76421 * t832 - F::new(36.0) * t14643 * t23238 + F::new(9.0) * t1553 * t18612 + F::new(60.0) * t23235 * t830 + F::new(3.0) * t23241 * t830;
    t77118
}
