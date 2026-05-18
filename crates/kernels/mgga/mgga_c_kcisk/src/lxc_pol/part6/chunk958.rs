//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 958/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk958<F: Float>(t23286: F, t23320: F, t23872: F, t28320: F, t28327: F, t28334: F, t28703: F, t28706: F, t28711: F, t28715: F, t28719: F, t28722: F) -> F {
    let t29971 = -F::new(0.52233124999999999998e-2) * t28320 - F::new(0.46429444444444444443e-2) * t23286 - F::new(0.34822083333333333333e-2) * t28327 + F::new(0.13928833333333333333e-1) * t28334 + F::new(0.17411041666666666666e-2) * t28703 - F::new(0.13928833333333333333e-1) * t28706 + F::new(0.34822083333333333333e-2) * t23320 - F::new(0.46429444444444444443e-2) * t23872 - F::new(0.11607361111111111111e-2) * t28711 - F::new(0.51072388888888888887e-1) * t28715 + F::new(0.34048259259259259259e-1) * t28719 - F::new(0.18571777777777777778e-1) * t28722;
    t29971
}
