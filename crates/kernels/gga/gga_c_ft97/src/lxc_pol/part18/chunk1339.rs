//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1339/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1339<F: Float>(t1359: F, t40424: F, t12334: F, t1901: F, t1647: F, t23657: F, t23671: F, t6656: F, t105776: F, t105779: F, t105784: F, t105788: F, t105791: F, t105795: F, t95242: F, t95254: F, t96105: F, t96107: F) -> (F, F, F) {
    let t105797 = t40424 * t1359;
    let t105799 = t1901 * t105797 * t12334;
    let t105804 = t23657 * t23671 * t6656 * t1647;
    let t105806 = 2.0 / 9.0 * t105776 + 10.0 / 27.0 * t105779 - 3.0 * t105784 - 8.0 / 3.0 * t105788 + 8.0 / 9.0 * t105791 - 3.0 / 4.0 * t105795 + 4.0 * t105799 - 8.0 / 9.0 * t95242 + t105804 / 6.0 - t96105 - t96107 + t95254;
    (t105799, t105804, t105806)
}
