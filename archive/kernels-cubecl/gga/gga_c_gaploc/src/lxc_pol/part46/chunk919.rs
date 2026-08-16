//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 919/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk919<F: Float>(t39904: F, t1063: F, t3152: F, t7974: F, t41809: F, t426: F, t2268: F, t535: F, t32067: F, t894: F, t12820: F, t2312: F) -> (F, F, F, F, F) {
    let t42874 = F::cast_from(0.71137516589190373998e-2_f64) * t39904;
    let t42877 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t3152 * t7974;
    let t42878 = t41809 * t426;
    let t42881 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t535 * t42878;
    let t42883 = t1063 * t894 * t32067;
    let t42885 = t2312 * t12820;
    (t42874, t42877, t42881, t42883, t42885)
}
