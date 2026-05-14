//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1280/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1280<F: Float>(t119625: F, t39693: F, t446: F, t119616: F, t9049: F, t119750: F, t27072: F, t5899: F, t119858: F, t23667: F, t105392: F, t23671: F, t3052: F, t6656: F, t119868: F, t119872: F, t119876: F, t119879: F, t119882: F, t119886: F) -> (F, F, F, F, F, F) {
    let t119889 = t446 * t39693 * t119625;
    let t119892 = t446 * t9049 * t119616;
    let t119895 = t5899 * t27072 * t119750;
    let t119898 = t5899 * t23667 * t119858;
    let t119902 = t105392 * t23671 * t6656 * t3052;
    let t119904 = -2.0 / 3.0 * t119868 + t119872 / 3.0 + 24.0 * t119876 + t119879 - t119882 + t119886 / 2.0 - 4.0 / 9.0 * t119889 + 2.0 / 9.0 * t119892 - 2.0 / 3.0 * t119895 - 2.0 / 3.0 * t119898 - t119902 / 3.0;
    (t119889, t119892, t119895, t119898, t119902, t119904)
}
