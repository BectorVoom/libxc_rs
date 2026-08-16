//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2343/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2343<F: Float>(t16311: F, t3788: F, t3850: F, t6936: F, t57554: F, t80915: F, t26233: F, t3858: F, t22783: F, t5310: F, t22760: F, t5234: F) -> (F, F, F, F, F, F) {
    let t91378 = t6936 * t3788 * t16311 * t3850;
    let t91381 = t6936 * t3788 * t57554;
    let t91383 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t80915;
    let t91384 = t26233 * t3858;
    let t91386 = t22783 * t5310;
    let t91387 = F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t91386;
    let t91388 = t5234 * t22760;
    (t91378, t91381, t91383, t91384, t91387, t91388)
}
