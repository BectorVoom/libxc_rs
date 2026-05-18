//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 575/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk575<F: Float>(t1336: F, t5541: F, t1907: F, t3856: F, t1334: F, t3861: F, t1897: F, t3873: F, t1319: F, t1324: F, t5481: F, t1903: F, t659: F) -> (F, F, F, F, F, F, F, F) {
    let t5543 = F::new(1.0) * t5541 * t1336;
    let t5545 = F::new(1.0) * t3856 * t1907;
    let t5546 = t1907 * t1334;
    let t5548 = F::new(2.0) * t3861 * t5546;
    let t5556 = t3873 * t1897;
    let t5557 = t5556 * t1319;
    let t5559 = t1324 * t5481;
    let t5562 = t659 * t1903;
    (t5543, t5545, t5546, t5548, t5556, t5557, t5559, t5562)
}
