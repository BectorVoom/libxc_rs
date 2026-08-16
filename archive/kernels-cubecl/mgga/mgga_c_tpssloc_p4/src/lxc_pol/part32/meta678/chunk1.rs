//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2117/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2117<F: Float>(t225: F, t27805: F, t24574: F, t27392: F, t1170: F, t2121: F, t27766: F, t2154: F, t45349: F, t27776: F, t95772: F, t11147: F, t497: F) -> (F, F, F, F, F, F) {
    let t95836 = t27805 * t225;
    let t95863 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27392;
    let t95866 = F::cast_from(0.54831135561607547884e-2_f64) * t2121 * t1170 * t27766;
    let t95884 = t45349 * t2154;
    let t95889 = F::cast_from(0.24369393582936687948e-2_f64) * t95772 * t27776;
    let t95890 = t497 * t11147;
    (t95836, t95863, t95866, t95884, t95889, t95890)
}
