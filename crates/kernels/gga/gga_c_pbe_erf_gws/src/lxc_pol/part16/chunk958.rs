//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 958/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk958<F: Float>(t7753: F, t7755: F, t7757: F, t7762: F, t7764: F, t7775: F, t7779: F, t7780: F, t7781: F, t7784: F, t7788: F, t7790: F, t7792: F, t7795: F, t7797: F, t7799: F) -> F {
    let t8452 = -t7753 + t7755 + t7757 - t7762 - t7764 + t7775 + t7779 + t7780 - t7781 - t7784 - t7788 - t7790 - t7792 + t7795 + t7797 - t7799;
    t8452
}
