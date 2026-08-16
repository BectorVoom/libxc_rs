//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1432/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1432<F: Float>(t12652: F, t65: F, t3961: F, t628: F, t12606: F, t31: F, t3967: F, t1409: F, t2244: F, t9287: F, t2267: F, t3966: F) -> (F, F, F, F, F, F, F) {
    let t12653 = t12652 * t65;
    let t12656 = t3961 * t628;
    let t12661 = t31 * t12606;
    let t12662 = t12661 * t65;
    let t12665 = t3967 * t628;
    let t12677 = t9287 * t1409 * t2244;
    let t12680 = t2267 * t3966;
    (t12653, t12656, t12661, t12662, t12665, t12677, t12680)
}
