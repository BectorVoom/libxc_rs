//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 708/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk708<F: Float>(t225: F, t3173: F, t368: F, t3068: F, t1058: F, t1926: F, t3158: F, t1942: F, t3082: F, t344: F, t40: F, t1009: F, sigma0: F) -> (F, F, F, F, F) {
    let t23394 = t225 * t3173;
    let t23417 = sigma0 * t368;
    let t23418 = t23417 * t3068;
    let t23419 = t1058 * t23418;
    let t23447 = t1926 * t3158 / F::cast_from(432.0_f64);
    let t23469 = t1942 * t3082 / F::cast_from(6912.0_f64);
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    (t23394, t23419, t23447, t23469, t23471)
}
