//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2872/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2872<F: Float>(t11465: F, t3015: F, t4707: F, t981: F, t11299: F, t15389: F, t2875: F, t11379: F, t1610: F, t2874: F, t11300: F, t15396: F, t41588: F) -> (F, F, F, F) {
    let t52150 = F::cast_from(0.31168546390226634765e3_f64) * t981 * t11465 * t4707 * t3015;
    let t52153 = F::cast_from(0.28947563097646563121e3_f64) * t11299 * t15389 * t2875;
    let t52156 = F::new(2.0) * t2874 * t1610 * t11379;
    let t52159 = F::cast_from(0.62071215503128080361e4_f64) * t41588 * t15396 * t11300;
    (t52150, t52153, t52156, t52159)
}
