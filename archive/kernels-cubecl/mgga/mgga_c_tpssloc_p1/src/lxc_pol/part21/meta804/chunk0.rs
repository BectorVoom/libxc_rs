//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2792/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2792<F: Float>(t59022: F, t12924: F, t16693: F, t13127: F, t16616: F, t2528: F, t12908: F, t16620: F, t12932: F, t4205: F, t47180: F, t47185: F) -> (F, F, F, F, F, F, F, F) {
    let t59023 = F::cast_from(48.0_f64) * t59022;
    let t59024 = t16693 * t12924;
    let t59025 = F::cast_from(48.0_f64) * t59024;
    let t59027 = F::cast_from(48.0_f64) * t16693 * t13127;
    let t59028 = t16616 * t2528;
    let t59029 = F::cast_from(0.17315859105681463759e2_f64) * t59028;
    let t59031 = F::cast_from(24.0_f64) * t12908 * t16620;
    let t59032 = t4205 * t12932;
    let t59033 = F::cast_from(16.0_f64) * t59032;
    let t59034 = F::cast_from(48.0_f64) * t47180;
    let t59035 = F::cast_from(24.0_f64) * t47185;
    (t59023, t59025, t59027, t59029, t59031, t59033, t59034, t59035)
}
