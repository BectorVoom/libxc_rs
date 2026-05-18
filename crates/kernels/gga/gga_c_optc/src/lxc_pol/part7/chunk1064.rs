//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1064/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1064<F: Float>(t2167: F, t6892: F, t2037: F, t7122: F, t7130: F, t127: F, t2024: F, t2113: F, t2124: F, t2126: F, t2168: F, t22173: F, t22203: F, t22788: F, t22792: F, t22859: F, t22872: F, t22875: F, t23081: F, t23083: F, t23085: F, t23098: F, t23105: F, t673: F, t675: F, t696: F, t6993: F) -> F {
    let t23109 = t2167 * t6892;
    let t23110 = t23109 * t2037;
    let t23117 = t7122 * t7130;
    let t23123 = -F::new(0.8463958349005185144e1) * t23081 - F::new(0.14604511302845113195e2) * t23083 - F::new(0.26079484469366273564e0) * t673 * t675 * t23085 * t127 + F::new(0.52158968938732547127e0) * t2113 * t675 * t23085 * t2024 + F::new(0.90685268025055555115e0) * t23098 * t696 * t22788 - F::new(0.10882232163006666614e1) * t6993 * t696 * t22792 + F::new(0.81136173904695073308e0) * t23105 - F::new(0.18137053605011111023e1) * t2168 * t22173 + F::new(0.19184972257745086326e2) * t23110 + F::new(0.10431793787746509425e1) * t2124 * t2126 * t22875 + F::new(0.24182738140014814697e0) * t2168 * t22872 + F::new(0.14604511302845113196e2) * t23117 - F::new(0.60456845350037036744e-1) * t2168 * t22203 - F::new(0.90685268025055555116e-1) * t2168 * t22859;
    t23123
}
