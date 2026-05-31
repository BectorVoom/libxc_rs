//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2863/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2863<F: Float>(t10696: F, t14643: F, t14648: F, t14652: F, t1553: F, t18392: F, t18435: F, t18599: F, t18612: F, t227: F, t23114: F, t23148: F, t23235: F, t23238: F, t23241: F, t4343: F, t4415: F, t4416: F, t5962: F, t76421: F, t775: F, t830: F, t832: F, t853: F) -> F {
    let t77118 = -F::cast_from(360.0_f64) * t10696 * t23114 * t4415 * t775 - F::cast_from(12.0_f64) * t23148 * t4415 * t775 * t853 + F::cast_from(180.0_f64) * t14648 * t18435 * t4415 - F::cast_from(36.0_f64) * t14652 * t4415 * t5962 - F::cast_from(36.0_f64) * t18392 * t4415 * t4416 + F::cast_from(180.0_f64) * t18599 * t4343 * t4415 + F::cast_from(3.0_f64) * t227 * t76421 * t832 - F::cast_from(36.0_f64) * t14643 * t23238 + F::cast_from(9.0_f64) * t1553 * t18612 + F::cast_from(60.0_f64) * t23235 * t830 + F::cast_from(3.0_f64) * t23241 * t830;
    t77118
}
