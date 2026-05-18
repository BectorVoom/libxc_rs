//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 834/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk834<F: Float>(t2795: F, t687: F, t2792: F, t286: F, t680: F, t2617: F, t2620: F, t195: F, t2987: F, t656: F, t4: F, t657: F, t901: F) -> (F, F, F, F, F) {
    let t11649 = t2795 * t687;
    let t11652 = F::new(0.61524113149298439947e4) * t286 * t2792 * t680 * t11649;
    let t11653 = t2617 * t2620;
    let t11657 = F::new(0.1301229756036208781e0) * t656 * t195 * t2987;
    let t11659 = t901 * t4 * t657;
    (t11649, t11652, t11653, t11657, t11659)
}
