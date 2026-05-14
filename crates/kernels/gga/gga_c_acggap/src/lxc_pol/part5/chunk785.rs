//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 785/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk785<F: Float>(t2632: F, t883: F, t2631: F, t2787: F, t286: F, t686: F, t2896: F, t98: F, t100: F, t2908: F, t2795: F, t687: F, t2792: F, t680: F, t2617: F, t2620: F) -> (F, F, F, F, F, F, F) {
    let t11597 = t883 * t2632;
    let t11602 = 0.69263436422725855036e2 * t286 * t686 * t2787 * t2631;
    let t11607 = 1.0 / t98 / t2896;
    let t11627 = 1.0 / t100 / t2908;
    let t11649 = t2795 * t687;
    let t11652 = 0.61524113149298439947e4 * t286 * t2792 * t680 * t11649;
    let t11653 = t2617 * t2620;
    (t11597, t11602, t11607, t11627, t11649, t11652, t11653)
}
