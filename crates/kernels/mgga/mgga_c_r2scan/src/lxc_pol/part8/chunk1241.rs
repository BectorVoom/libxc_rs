//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1241/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1241<F: Float>(t761: F, t7760: F, t6082: F, t980: F, t20470: F, t7418: F, t2582: F, t2583: F, t6848: F, t146: F, t2145: F, t2832: F, t6103: F, t5100: F, t8071: F, t6407: F) -> (F, F, F, F, F, F, F, F) {
    let t26976 = t7760 * t761;
    let t27004 = t980 * t6082;
    let t27006 = t20470 * t7418;
    let t27022 = t2582 * t6848 * t2583;
    let t27023 = 0.12713391885412927226e1 * t27022;
    let t27067 = t146 * t2145 * t2832;
    let t27074 = t980 * t6103;
    let t27077 = t5100 * t8071;
    let t27078 = 0.4939086887201633699e-1 * t27077;
    let t27079 = t6407 * t8071;
    (t26976, t27004, t27006, t27023, t27067, t27074, t27078, t27079)
}
