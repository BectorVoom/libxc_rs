//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2023/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2023<F: Float>(t2030: F, t47567: F, t26069: F, t94806: F, t26054: F, t9686: F, t25877: F, t94801: F, t1419: F, t786: F, t2023: F, t4075: F) -> (F, F, F, F, F, F, F) {
    let t94867 = F::cast_from(0.81814717454467823679e-4_f64) * t47567 * t2030;
    let t94876 = t26069 * t94806;
    let t94884 = t26054 * t9686;
    let t94886 = t94801 * t25877;
    let t94889 = t786 * t1419;
    let t94890 = t94889 * t25877;
    let t94901 = t786 * t2023 * t4075;
    (t94867, t94876, t94884, t94886, t94889, t94890, t94901)
}
