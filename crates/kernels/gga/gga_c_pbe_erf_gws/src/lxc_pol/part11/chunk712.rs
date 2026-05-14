//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 712/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk712<F: Float>(t11: F, t12481: F, t10823: F, t10825: F, t10827: F, t12462: F, t12466: F, t12470: F, t12474: F, t12478: F, t4940: F, t7374: F, t173: F, t184: F, t199: F, t12350: F, t5063: F) -> (F, F, F, F, F, F) {
    let t12482 = t11 * t12481;
    let t12484 = t4940 + 0.25188888888888888889e-2 * t7374 - 0.12594444444444444445e-2 * t10823 + 0.37783333333333333335e-2 * t10825 - 0.18891666666666666667e-2 * t10827 + 0.20990740740740740742e-2 * t12462 - 0.75566666666666666669e-2 * t12466 + 0.37783333333333333335e-2 * t12470 + 0.11335e-1 * t12474 - 0.11335e-1 * t12478 + 0.18891666666666666667e-2 * t12482;
    let t12485 = t173 * t12484;
    let t12486 = t12485 * t184;
    let t12488 = 2.0 / 15.0 * t12486 * t199;
    let t12493 = t5063 * t12350;
    (t12482, t12484, t12485, t12486, t12488, t12493)
}
