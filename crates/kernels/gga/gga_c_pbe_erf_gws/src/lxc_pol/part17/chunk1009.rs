//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1009/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1009<F: Float>(t6080: F, t4826: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4856: F, t4864: F, t8022: F, t8024: F, t8026: F, t8027: F, t8028: F, t8031: F, t8032: F, t8033: F, t8034: F, t8035: F) -> F {
    let t9047 = F::new(0.13692109613355555556e1) * t6080;
    let t9048 = -t8022 + t8024 - t8026 - t8027 + t4826 + t8028 - t8031 - t4837 - t4840 - t4843 + t4846 + t4849 + t8032 - t8033 + t9047 - t4856 + t8034 + t8035 - t4864;
    t9048
}
