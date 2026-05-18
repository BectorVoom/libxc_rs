//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 991/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk991<F: Float>(t10847: F, t10851: F, t10857: F, t11428: F, t11432: F, t11433: F, t11817: F, t11819: F, t11822: F, t11826: F, t11831: F, t11843: F) -> (F, F) {
    let t12188 = F::new(0.47609969197673950973e-2) * t11817 + F::new(0.10975748638225852664e0) * t11819 + F::new(0.13099107994629972538e-1) * t11822 + F::new(0.13099107994629972538e-1) * t11826 - t10847 - t10851 - t11428 - F::new(0.97574405393827830187e-2) * t10857 + F::new(0.43663693315433241794e-2) * t11831 + t11432 + t11433;
    let t12192 = F::new(0.23115257973478049502e0) * t11843;
    (t12188, t12192)
}
