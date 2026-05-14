//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1267/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1267<F: Float>(t119657: F, t27072: F, t5899: F, t4714: F, t5842: F, t1369: F, t2112: F, t28: F, t1359: F, t16919: F, t4778: F, t586: F, t5890: F, t3408: F, t6615: F, t105417: F, t105434: F, t119642: F, t119645: F, t119649: F, t119653: F, t119656: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119659 = t5899 * t27072 * t119657;
    let t119661 = t5842 * t4714;
    let t119664 = t1369 * t28 * t2112 * t119661;
    let t119665 = t1359 * t16919;
    let t119668 = t1369 * t28 * t2112 * t119665;
    let t119672 = t5890 * t28 * t586 * t5842 * t4778;
    let t119674 = t6615 * t3408;
    let t119677 = t1369 * t28 * t2112 * t119674;
    let t119679 = -t119642 / 12.0 + t119645 + t105417 + 2.0 * t119649 - t119653 - t119656 + t105434 + 2.0 / 9.0 * t119659 + t119664 + t119668 + t119672 / 4.0 + 2.0 * t119677;
    (t119659, t119661, t119664, t119665, t119668, t119672, t119674, t119677, t119679)
}
