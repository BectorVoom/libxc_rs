//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1082/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1082(t1473: f64, t1497: f64, t5636: f64, t751: f64, t5607: f64, t5611: f64, t5615: f64, t101: f64, t1593: f64, t19174: f64, t19177: f64, t19179: f64, t19182: f64, t19185: f64, t19187: f64, t19191: f64, t19195: f64, t19199: f64, t19203: f64, t19206: f64, t19209: f64, t19454: f64, t2036: f64, t2857: f64, t523: f64, t526: f64, t5629: f64, t5645: f64, t5650: f64) -> f64 {
    let t19458 = 0.31931290694012290916e0_f64 * t1473 * t1497;
    let t19459 = t751 * t5636;
    let t19461 = t751 * t5607;
    let t19463 = t751 * t5611;
    let t19466 = 0.79828226735030727292e-1_f64 * t751 * t5615;
    let t19469 = -t19174 - 0.36991419282863461287e1_f64 * t19177 + 0.78054266140918933351e0_f64 * t19179 + t19182 + 0.11890099055206112556e1_f64 * t19185 - 3.0_f64 * t523 * t19187 - 24.0_f64 * t5650 * t19191 - 12.0_f64 * t5650 * t19195 - 0.3486808982146430324e-2_f64 * t19199 - t19203 - 0.11622696607154767747e-2_f64 * t19206 - 0.3486808982146430324e-2_f64 * t19209 + 24.0_f64 * t2857 * t2036 * t5645 + t101 * t19454 * t526 - t19458 + 0.79828226735030727292e-1_f64 * t19459 + 0.23948468020509218188e0_f64 * t19461 + 0.23948468020509218188e0_f64 * t19463 + t19466 - 3.0_f64 * t1593 * t5629;
    t19469
}
