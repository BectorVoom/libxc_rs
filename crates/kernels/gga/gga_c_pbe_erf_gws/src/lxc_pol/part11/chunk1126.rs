//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1126/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1126<F: Float>(t41245: F, t47372: F, t626: F, t11: F, t625: F, t47377: F, t5063: F, t1691: F, t1642: F, t47733: F, t17900: F, t30955: F, t30957: F, t30962: F, t32373: F, t32375: F, t41888: F, t41890: F) -> (F, F, F, F, F, F, F, F) {
    let t47928 = F::new(64.0) / F::new(45.0) * t41245;
    let t47929 = t626 * t47372;
    let t47931 = t11 * t625 * t47929;
    let t47940 = t5063 * t47377;
    let t47942 = t11 * t1691 * t47940;
    let t47944 = t1642 * t47733;
    let t47946 = t11 * t1691 * t47944;
    let t47948 = -F::cast_from(0.35991666666666666667e-1_f64) * t47931 + t17900 + F::cast_from(0.17777777777777777778e-1_f64) * t41888 - F::cast_from(0.10666666666666666667e0_f64) * t41890 - F::cast_from(0.63985185185185185184e-1_f64) * t30955 - F::cast_from(0.95977777777777777776e-1_f64) * t30957 + F::cast_from(0.19195555555555555555e0_f64) * t30962 - F::cast_from(0.44444444444444444445e-1_f64) * t32373 - F::cast_from(0.14814814814814814815e-1_f64) * t32375 - F::cast_from(0.86380000000000000002e0_f64) * t47942 - F::cast_from(0.71983333333333333335e-1_f64) * t47946;
    (t47928, t47929, t47931, t47940, t47942, t47944, t47946, t47948)
}
