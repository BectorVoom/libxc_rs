//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1277;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1278;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta340<F: Float>(t1409: F, t2250: F, t65: F, t3966: F, t607: F, t3961: F, t628: F, t12606: F, t31: F, t3967: F, t2244: F, t9287: F, t2267: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t12648 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1277::<F>(t1409, t2250);
        let (t12649, t12652) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1278::<F>(t12648, t65, t3966, t607);
        let (t12653, t12656, t12661, t12662, t12665, t12677, t12680) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1279::<F>(t12652, t65, t3961, t628, t12606, t31, t3967, t1409, t2244, t9287, t2267, t3966);
    (t12648, t12649, t12652, t12653, t12656, t12661, t12662, t12665, t12677, t12680)
}
