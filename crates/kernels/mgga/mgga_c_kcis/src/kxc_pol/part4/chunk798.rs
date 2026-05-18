//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 798/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk798<F: Float>(t278: F, t417: F, t4967: F, t4768: F, t1001: F, t286: F, t1700: F, t1706: F, t285: F, t2870: F, t2872: F, t2879: F, t2882: F, t2885: F, t4937: F, t4940: F, t4944: F, t4948: F, t4953: F, t4959: F, t4963: F, t984: F, t991: F) -> (F, F, F, F, F) {
    let t288 = F::new(0.0) < t278;
    let t4968 = t417 * t4967;
    let t4972 = piecewise3::<f64>(t288, t4768, -t4768);
    let t4973 = t1001 * t4972;
    let t4974 = t286 * t4973;
    let t4977 = -t2870 / F::new(108.0) - t2879 + t2882 / F::new(864.0) - t2885 / F::new(288.0) - t2872 * t1700 / F::new(108.0) + t4937 / F::new(864.0) + t991 * t4940 / F::new(216.0) - t991 * t4944 / F::new(288.0) - t991 * t4948 / F::new(144.0) + t991 * t4953 / F::new(144.0) + t984 * t1706 / F::new(36.0) - t4959 / F::new(288.0) - t991 * t4963 / F::new(288.0) + t991 * t4968 / F::new(48.0) - t285 * t4974 / F::new(96.0);
    (t4968, t4972, t4973, t4974, t4977)
}
