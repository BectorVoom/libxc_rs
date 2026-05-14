//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 993/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk993<F: Float>(t47377: F, t5063: F, t11: F, t1691: F, t1642: F, t47733: F, t17900: F, t30955: F, t30957: F, t30962: F, t32373: F, t32375: F, t41888: F, t41890: F, t47931: F, t5002: F) -> (F, F, F, F, F, F) {
    let t47940 = t5063 * t47377;
    let t47942 = t11 * t1691 * t47940;
    let t47944 = t1642 * t47733;
    let t47946 = t11 * t1691 * t47944;
    let t47948 = -0.35991666666666666667e-1 * t47931 + t17900 + 0.17777777777777777778e-1 * t41888 - 0.10666666666666666667e0 * t41890 - 0.63985185185185185184e-1 * t30955 - 0.95977777777777777776e-1 * t30957 + 0.19195555555555555555e0 * t30962 - 0.44444444444444444445e-1 * t32373 - 0.14814814814814814815e-1 * t32375 - 0.86380000000000000002e0 * t47942 - 0.71983333333333333335e-1 * t47946;
    let t47949 = t5002 * t47377;
    (t47940, t47942, t47944, t47946, t47948, t47949)
}
