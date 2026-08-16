//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1117;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1118;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta270<F: Float>(t1799: F, t6968: F, t6637: F, t6888: F, t5335: F, t550: F, t6976: F, t1992: F, t1834: F, t1998: F, t214: F, t1985: F, t2031: F, t7445: F, t5: F, t1860: F, t2032: F, t7026: F, t7034: F, t7428: F, t7432: F, t7435: F, t112: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1117::<F>(t1799, t6968, t6637, t6888, t5335, t550, t6976, t1992, t1834, t1998, t214, t1985);
        let t7782 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1118::<F>(t2031, t7445);
        let (t7786, t7787) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1119::<F>(t5, t1860, t2032, t7026, t7034, t7428, t7432, t7435, t7782, t112);
    (t7732, t7733, t7734, t7736, t7737, t7738, t7740, t7741, t7742, t7782, t7786, t7787)
}
