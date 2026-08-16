//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 215/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk215<F: Float>(t291: F, t474: F, t4: F, t773: F, t139: F, t286: F, t124: F, t495: F, t498: F, t288: F, t483: F, t486: F, t494: F) -> (F, F, F, F, F, F, F, F) {
    let t800 = t474 * t291;
    let t801 = t773 * t4;
    let t802 = t800 * t801;
    let t807 = t286 * t139;
    let t808 = t807 * t124;
    let t811 = t286 * t495;
    let t812 = t811 * t498;
    let t815 = t807 * t4;
    let t818 = -F::cast_from(0.97071966386951317368e-2_f64) * t483 * t288 - F::cast_from(0.12133995798368914671e-2_f64) * t486 * t808 + F::cast_from(0.12133995798368914671e-3_f64) * t494 * t812 - F::cast_from(0.21574244529499930286e-3_f64) * t494 * t815;
    (t800, t801, t802, t808, t811, t812, t815, t818)
}
