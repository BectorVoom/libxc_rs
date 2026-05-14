//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1087/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1087<F: Float>(t27391: F, t604: F, t26564: F, t5766: F, t1349: F, t26779: F, t376: F, t1013: F, t358: F, t22572: F, t23705: F, t26706: F, t23715: F, t26701: F, t26696: F, t26722: F) -> (F, F, F, F, F, F, F, F) {
    let t104623 = t27391 * t604;
    let t104627 = t5766 * t26564 / 9.0;
    let t104632 = t1349 * t376 * t26779 / 9.0;
    let t104637 = t1013 * t358;
    let t104647 = 0.22226000364197530866e-1 * t23705 * t22572 * t26706;
    let t104658 = 0.22226000364197530866e-1 * t23715 * t22572 * t26701;
    let t104663 = t23705 * t22572 * t26696;
    let t104682 = t23715 * t22572 * t26722;
    (t104623, t104627, t104632, t104637, t104647, t104658, t104663, t104682)
}
