//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 792/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk792<F: Float>(t1091: F, t2619: F, t724: F, t1882: F, t3844: F, t3821: F, t713: F, t2574: F, t265: F, t766: F, t729: F, t762: F, t1175: F, t2409: F, t2413: F, t3897: F) -> (F, F, F, F, F, F) {
    let t14048 = t724 * t2619 * t1091;
    let t14052 = 2.0 / 9.0 * t1882 * t3844;
    let t14053 = t3821 * t713;
    let t14055 = t2574 * t265 * t14053;
    let t14058 = t3821 * t766;
    let t14060 = t729 * t762 * t14058;
    let t14064 = t724 * t1175 * t2409;
    let t14067 = t3897 * t2413;
    (t14048, t14052, t14055, t14060, t14064, t14067)
}
