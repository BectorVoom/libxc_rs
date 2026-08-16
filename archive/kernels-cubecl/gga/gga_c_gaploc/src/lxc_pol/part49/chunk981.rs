//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 981/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk981<F: Float>(t39895: F, t39897: F, t39899: F, t39901: F, t39904: F, t1063: F, t3152: F, t7974: F, t41809: F, t426: F, t2268: F, t535: F) -> (F, F, F, F, F, F, F) {
    let t42870 = F::cast_from(0.23712505529730124666e-2_f64) * t39895;
    let t42871 = F::cast_from(0.31616674039640166221e-2_f64) * t39897;
    let t42872 = F::cast_from(0.23712505529730124666e-2_f64) * t39899;
    let t42873 = F::cast_from(0.94850022118920498664e-2_f64) * t39901;
    let t42874 = F::cast_from(0.71137516589190373998e-2_f64) * t39904;
    let t42877 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t3152 * t7974;
    let t42878 = t41809 * t426;
    let t42881 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t535 * t42878;
    (t42870, t42871, t42872, t42873, t42874, t42877, t42881)
}
