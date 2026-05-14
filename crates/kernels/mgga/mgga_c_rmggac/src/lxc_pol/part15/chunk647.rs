//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 647/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk647<F: Float>(t10033: F, t530: F, t8876: F, t1945: F, t1986: F, t675: F, t1859: F, t194: F, t201: F, t1979: F, t1982: F, t128: F, t1864: F, t118: F, t7408: F, t1737: F, t645: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10034 = 0.85129199786595678796e-5 * t10033;
    let t10036 = t530 * t8876;
    let t10037 = 0.4726e1 * t10036;
    let t10040 = t1986 * t1945;
    let t10041 = t675 * t10040;
    let t10042 = 0.85129199786595678796e-5 * t10041;
    let t10043 = t194 * t1859;
    let t10044 = t10043 * t201;
    let t10046 = t10044 * t1979 * t1982;
    let t10047 = 0.42564599893297839398e-5 * t10046;
    let t10048 = t128 * t1864;
    let t10049 = t118 * t10048;
    let t10050 = t1986 * t10049;
    let t10051 = t7408 * t10050;
    let t10052 = 0.11971293719990017331e-4 * t10051;
    let t10053 = t645 * t1737;
    (t10034, t10037, t10040, t10042, t10043, t10044, t10047, t10050, t10052, t10053)
}
