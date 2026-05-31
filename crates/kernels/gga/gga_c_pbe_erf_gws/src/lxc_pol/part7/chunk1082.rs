//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1082/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1082<F: Float>(t1473: F, t1497: F, t5636: F, t751: F, t5607: F, t5611: F, t5615: F, t101: F, t1593: F, t19174: F, t19177: F, t19179: F, t19182: F, t19185: F, t19187: F, t19191: F, t19195: F, t19199: F, t19203: F, t19206: F, t19209: F, t19454: F, t2036: F, t2857: F, t523: F, t526: F, t5629: F, t5645: F, t5650: F) -> F {
    let t19458 = F::cast_from(0.31931290694012290916e0_f64) * t1473 * t1497;
    let t19459 = t751 * t5636;
    let t19461 = t751 * t5607;
    let t19463 = t751 * t5611;
    let t19466 = F::cast_from(0.79828226735030727292e-1_f64) * t751 * t5615;
    let t19469 = -t19174 - F::cast_from(0.36991419282863461287e1_f64) * t19177 + F::cast_from(0.78054266140918933351e0_f64) * t19179 + t19182 + F::cast_from(0.11890099055206112556e1_f64) * t19185 - F::cast_from(3.0_f64) * t523 * t19187 - F::cast_from(24.0_f64) * t5650 * t19191 - F::cast_from(12.0_f64) * t5650 * t19195 - F::cast_from(0.3486808982146430324e-2_f64) * t19199 - t19203 - F::cast_from(0.11622696607154767747e-2_f64) * t19206 - F::cast_from(0.3486808982146430324e-2_f64) * t19209 + F::cast_from(24.0_f64) * t2857 * t2036 * t5645 + t101 * t19454 * t526 - t19458 + F::cast_from(0.79828226735030727292e-1_f64) * t19459 + F::cast_from(0.23948468020509218188e0_f64) * t19461 + F::cast_from(0.23948468020509218188e0_f64) * t19463 + t19466 - F::cast_from(3.0_f64) * t1593 * t5629;
    t19469
}
