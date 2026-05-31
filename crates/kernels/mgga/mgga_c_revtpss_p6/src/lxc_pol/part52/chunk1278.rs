//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1278/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1278<F: Float>(t28184: F, t8698: F, t1353: F, t26405: F, t28167: F, t34301: F, t32626: F, t7935: F, t102019: F, t1937: F, t111018: F, t28653: F, t6993: F) -> (F, F, F, F, F, F) {
    let t128970 = F::cast_from(3.0_f64) * t8698 * t28184;
    let t128974 = F::cast_from(6.0_f64) * t28167 * t26405 * t34301 * t1353;
    let t128975 = t32626 * t7935;
    let t128977 = F::cast_from(2.0_f64) * t102019 * t1937;
    let t128979 = F::cast_from(2.0_f64) * t111018 * t1937;
    let t128981 = F::cast_from(2.0_f64) * t28653 * t6993;
    (t128970, t128974, t128975, t128977, t128979, t128981)
}
