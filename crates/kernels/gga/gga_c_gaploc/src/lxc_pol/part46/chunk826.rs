//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 826/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk826<F: Float>(t41698: F, t39968: F, t10122: F, t1445: F, t2293: F, t574: F, t12939: F, t1407: F, t41650: F, t41654: F, t41657: F, t41661: F, t41664: F, t41667: F, t41670: F, t41672: F, t41675: F, t41677: F, t41681: F, t41684: F, t41687: F, t41690: F, t41692: F, t41697: F) -> F {
    let t41699 = F::new(0.1022478025437886658e1) * t41698;
    let t41700 = F::new(0.19171462976960374838e1) * t39968;
    let t41703 = t574 * t1445 * t10122 * t2293;
    let t41705 = t1407 * t12939;
    let t41706 = F::new(0.15976219147466979032e-1) * t41705;
    let t41707 = t41650 + t41654 - t41657 + t41661 - F::new(0.79445533226334281487e-1) * t41664 - t41667 - t41670 - F::new(0.76685851907841499352e0) * t41672 - t41675 + t41677 + t41681 - t41684 - t41687 + t41690 - t41692 - t41697 + t41699 - t41700 - F::new(0.92023022289409799224e1) * t41703 - t41706;
    t41707
}
