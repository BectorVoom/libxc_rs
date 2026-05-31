//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2955/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2955<F: Float>(t15547: F, t6223: F, t1642: F, t64510: F, t23453: F, t3022: F, t1100: F, t23571: F, t41937: F, t5023: F, t77634: F, t77636: F, t77639: F, t77641: F, t77643: F, t77645: F, t77647: F) -> (F, F, F, F) {
    let t78405 = F::cast_from(0.17544670867903938621e1_f64) * t15547 * t6223;
    let t78411 = F::cast_from(0.17544670867903938621e1_f64) * t64510 * t1642;
    let t78413 = F::cast_from(0.10389515463408878255e3_f64) * t3022 * t23453;
    let t78414 = -F::cast_from(6.0_f64) * t1100 * t23571 * t41937 * t5023 + t77634 - t77636 + t77639 + t77641 + t77643 - t77645 + t77647 - t78405 - t78411 + t78413;
    (t78405, t78411, t78413, t78414)
}
