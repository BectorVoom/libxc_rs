//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1303/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1303<F: Float>(t23809: F, t3347: F, t2071: F, t22591: F, t26738: F, t104912: F, t104917: F, t104932: F, t105102: F, t105106: F, t105110: F, t105117: F, t105124: F, t105127: F, t105130: F, t2034: F, t23810: F, t23866: F, t23869: F, t26617: F, t3394: F, t40081: F, t48660: F, t5784: F, t5785: F, t5802: F, t6593: F, t8838: F, t8859: F, t93178: F, t94608: F, t94892: F) -> (F, F) {
    let t105135 = t3347 * t23809;
    let t105143 = t22591 * t26738 * t2071;
    let t105148 = -0.37222487257520572791e2 * t23866 * t105102 - 0.21895580739717983994e1 * t23810 * t105106 - 0.21895580739717983995e1 * t8859 * t105110 + 0.45306850413028723348e0 * t48660 * t5784 * t6593 - 0.56502890877451119022e-1 * t94608 * t105117 - 0.10947790369858991998e1 * t2034 * t93178 * t94892 * t3394 - 0.66678001092592592594e-1 * t105124 - 0.66678001092592592594e-1 * t105127 - 0.45306850413028723348e0 * t5802 * t105130 + 0.45306850413028723348e0 * t5785 * t105130 - 0.21895580739717983994e1 * t105135 * t26617 - 0.90613700826057446696e0 * t40081 * t104912 - 0.48327307107230638238e1 * t23869 * t104917 + 0.45306850413028723348e0 * t8838 * t105143 - 0.48327307107230638237e1 * t23869 * t104932;
    (t105143, t105148)
}
