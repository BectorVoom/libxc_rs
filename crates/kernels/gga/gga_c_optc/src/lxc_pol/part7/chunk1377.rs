//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1377/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1377<F: Float>(t3236: F, t8434: F, t438: F, t8905: F, t1028: F, t11596: F, t1162: F, t1179: F, t26122: F, t27346: F, t27353: F, t27356: F, t27358: F, t27363: F, t27365: F, t27366: F, t27370: F, t27374: F, t27378: F, t27382: F, t27383: F, t27385: F, t27389: F, t3087: F, t3092: F, t3107: F, t3146: F, t3234: F, t3235: F, t3244: F, t3245: F, t4387: F, t8511: F, t894: F, t8951: F, t9102: F, t9116: F, t914: F) -> (F, F) {
    let t27403 = t8434 * t3236;
    let t27414 = t8905 * t438;
    let t27415 = t27414 * t1028;
    let t27419 = F::cast_from(0.18137053605011111024e0_f64) * t1179 * t894 * t8511 * t27346 - F::cast_from(0.1039653020352937208e2_f64) * t27353 + F::cast_from(0.6717427261115226305e-1_f64) * t27356 + F::cast_from(0.47123383072914168269e1_f64) * t3244 * t11596 * t27358 + F::cast_from(0.519826510176468604e2_f64) * t27363 + F::cast_from(0.20408653907080965924e7_f64) * t9116 * t27365 * t27366 + F::cast_from(0.34014423178468276542e6_f64) * t9102 * t27365 * t27370 + F::cast_from(0.2339219295794108718e2_f64) * t3234 * t3235 * t27374 + F::cast_from(0.389869882632351453e2_f64) * t3234 * t4387 * t27378 - F::cast_from(0.45352564237957702055e6_f64) * t27382 * t27383 * t3107 * t27385 - F::cast_from(0.1343485452223045261e0_f64) * t27389 + F::cast_from(0.1343485452223045261e0_f64) * t1179 * t894 * t8951 * t27346 + F::cast_from(0.33587136305576131525e-1_f64) * t1179 * t894 * t3146 * t26122 - F::cast_from(0.23181763972770020945e0_f64) * t1162 * t914 * t3092 * t26122 + F::cast_from(0.15146801702008125515e1_f64) * t3244 * t3245 * t27403 + F::cast_from(0.15454509315180013964e0_f64) * t1162 * t914 * t3087 * t26122 + F::cast_from(0.1559479530529405812e2_f64) * t3234 * t3235 * t27403 + F::cast_from(0.1559479530529405812e2_f64) * t3234 * t3235 * t27415;
    (t27415, t27419)
}
