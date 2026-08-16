//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3097/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3097<F: Float>(t50846: F, t50848: F, t50853: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F, t63997: F, t64003: F, t64006: F, t64009: F) -> F {
    let t64181 = -F::cast_from(0.10805407407407407407e0_f64) * t63918 - F::cast_from(0.69463333333333333334e-1_f64) * t63921 - F::cast_from(0.34731666666666666667e-1_f64) * t63924 - F::cast_from(0.20839e0_f64) * t63927 + F::cast_from(0.46308888888888888889e-1_f64) * t63930 + F::cast_from(0.55570666666666666666e0_f64) * t63933 + F::cast_from(0.62517e0_f64) * t63936 + F::cast_from(0.250068e1_f64) * t63939 + F::cast_from(0.3529725e1_f64) * t63997 - F::cast_from(0.61745185185185185187e0_f64) * t50846 - F::cast_from(0.13892666666666666667e0_f64) * t50848 + F::cast_from(0.4630888888888888889e0_f64) * t50853 - F::cast_from(0.83356000000000000001e0_f64) * t64003 + F::cast_from(0.250068e1_f64) * t64006 + F::cast_from(0.6311625e0_f64) * t64009;
    t64181
}
