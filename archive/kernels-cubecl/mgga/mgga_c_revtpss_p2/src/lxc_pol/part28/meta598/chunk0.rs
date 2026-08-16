//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2073/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2073<F: Float>(t10073: F, t1444: F, t2029: F, t25929: F, t26041: F, t9664: F, t2030: F, t47567: F, t26069: F, t94806: F, t1426: F, t94609: F) -> (F, F, F, F, F) {
    let t94857 = t10073 * t25929 * t2029 * t1444;
    let t94865 = F::cast_from(0.46263278077393568556e-2_f64) * t26041 * t9664;
    let t94867 = F::cast_from(0.81814717454467823679e-4_f64) * t47567 * t2030;
    let t94876 = t26069 * t94806;
    let t94878 = t94609 * t1426;
    (t94857, t94865, t94867, t94876, t94878)
}
