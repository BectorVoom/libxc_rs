//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2020/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2020<F: Float>(t2332: F, t81442: F, t22470: F, t2358: F, t63: F, t9365: F, t193: F, t201: F, t6665: F, t23285: F, t2752: F, t10143: F) -> (F, F, F, F, F, F) {
    let t81443 = t81442 * t2332;
    let t81445 = t22470 * t2358;
    let t81446 = t63 * t9365;
    let t81483 = t193 * t201 * t6665;
    let t81525 = t23285 * t2752;
    let t81539 = t6665 * t10143;
    (t81443, t81445, t81446, t81483, t81525, t81539)
}
