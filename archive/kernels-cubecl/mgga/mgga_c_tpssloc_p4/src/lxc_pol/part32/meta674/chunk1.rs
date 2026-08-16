//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2110/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2110<F: Float>(t27674: F, t3548: F, t15753: F, t7310: F, t27608: F, t7321: F, t1222: F, t27586: F, t3540: F, t8049: F, t2132: F, t2136: F, t3966: F) -> (F, F, F, F, F, F) {
    let t95511 = t27674 * t3548 / F::cast_from(162.0_f64);
    let t95512 = t7310 * t15753;
    let t95515 = F::cast_from(0.20186378047070195428e-3_f64) * t27608 * t7321;
    let t95517 = t27586 * t1222 / F::cast_from(1152.0_f64);
    let t95520 = t8049 * t3540;
    let t95540 = F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t3966 * t2136;
    (t95511, t95512, t95515, t95517, t95520, t95540)
}
