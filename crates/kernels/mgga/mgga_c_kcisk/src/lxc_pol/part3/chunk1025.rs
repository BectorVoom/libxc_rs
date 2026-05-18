//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1025/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1025<F: Float>(t14556: F, t14558: F, t14561: F, t14563: F, t14565: F, t14568: F, t14571: F, t14575: F, t14579: F, t14582: F, t14584: F, t14586: F, t14589: F, t14593: F, t14596: F) -> F {
    let t15165 = F::new(0.5625e0) * t14556 + F::new(0.275e1) * t14558 + F::new(0.71944444444444444444e-1) * t14561 - F::new(0.40468749999999999999e-1) * t14563 - F::new(0.75e0) * t14565 + F::new(0.625e-1) * t14568 + F::new(0.16666666666666666666e0) * t14571 + F::new(0.27777777777777777777e-1) * t14575 + F::new(0.101171875e-1) * t14579 - F::new(0.375e0) * t14582 + F::new(0.161875e0) * t14584 - F::new(0.10252083333333333334e1) * t14586 - F::new(0.275e1) * t14589 - F::new(0.43166666666666666667e0) * t14593 - F::new(0.80937499999999999999e-1) * t14596;
    t15165
}
