//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 679/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk679<F: Float>(t1336: F, t3856: F, t1314: F, t465: F, t455: F, t1334: F) -> (F, F, F, F) {
    let t3858 = F::new(2.0) * t3856 * t1336;
    let t3859 = t1314 * t465;
    let t3860 = F::new(1.0) / t3859;
    let t3861 = t455 * t3860;
    let t3862 = t1334 * t1334;
    (t3858, t3860, t3861, t3862)
}
