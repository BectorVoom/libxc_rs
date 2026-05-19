//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 751/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk751<F: Float>(t486: F, t5727: F, t613: F, t5623: F, t1378: F, t286: F, t1368: F, t1373: F, t1382: F, t1930: F, t3969: F, t3972: F, t3975: F, t493: F, t5689: F, t5691: F, t5699: F, t5702: F, t5706: F, t5710: F, t5715: F, t5719: F, t5723: F) -> (F, F, F, F, F) {
    let t495 = F::new(0.0) < t486;
    let t5728 = t613 * t5727;
    let t5732 = piecewise3::<F>(t495, t5623, -t5623);
    let t5733 = t1378 * t5732;
    let t5734 = t286 * t5733;
    let t5737 = -t5689 / F::new(108.0) - t5691 * t1373 / F::new(108.0) + t1930 * t1382 / F::new(36.0) - t3969 + t3972 / F::new(864.0) - t3975 / F::new(288.0) + t5699 / F::new(864.0) + t1368 * t5702 / F::new(216.0) - t1368 * t5706 / F::new(288.0) - t1368 * t5710 / F::new(144.0) - t1368 * t5715 / F::new(144.0) - t5719 / F::new(288.0) - t1368 * t5723 / F::new(288.0) + t1368 * t5728 / F::new(48.0) - t493 * t5734 / F::new(96.0);
    (t5728, t5732, t5733, t5734, t5737)
}
