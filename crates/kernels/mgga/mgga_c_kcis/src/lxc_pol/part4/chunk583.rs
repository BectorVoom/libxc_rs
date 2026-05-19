//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 583/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk583<F: Float>(t278: F, t3038: F, t1001: F, t286: F, t1005: F, t285: F, t2867: F, t2870: F, t2872: F, t2879: F, t2882: F, t2885: F, t2891: F, t2896: F, t2901: F, t2905: F, t2913: F, t293: F, t984: F, t991: F, t996: F) -> (F, F, F) {
    let t288 = F::new(0.0) < t278;
    let t3040 = piecewise3::<F>(t288, t3038, -t3038);
    let t3041 = t1001 * t3040;
    let t3042 = t286 * t3041;
    let t3045 = F::new(11.0) / F::new(108.0) * t2867 * t293 - t2870 / F::new(54.0) - t2872 * t996 / F::new(54.0) + t984 * t1005 / F::new(18.0) - t2879 + t2882 / F::new(432.0) - t2885 / F::new(144.0) + t991 * t2891 / F::new(216.0) - t991 * t2896 / F::new(144.0) - t991 * t2901 / F::new(144.0) + t991 * t2905 / F::new(288.0) + t285 * t2913 / F::new(48.0) - t285 * t3042 / F::new(96.0);
    (t3040, t3041, t3045)
}
