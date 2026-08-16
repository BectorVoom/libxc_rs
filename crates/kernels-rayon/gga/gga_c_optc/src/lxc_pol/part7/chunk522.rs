//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 522/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk522(t1: f64, t2606: f64, t297: f64, t313: f64, t2246: f64, t301: f64, t300: f64, t885: f64, t889: f64, t2574: f64, t2577: f64, t2581: f64, t2583: f64, t2588: f64, t2591: f64, t2598: f64, t2603: f64, t289: f64, t314: f64, t862: f64, t874: f64, t893: f64, t899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2607 = t2606 * t1;
    let t2608 = t2607 * t297;
    let t2609 = t313 * t2608;
    let t2612 = t301 * t2246;
    let t2613 = t300 * t2612;
    let t2616 = t885 * t889;
    let t2618 = 11.0_f64 / 108.0_f64 * t2574 * t289 - t2577 / 54.0_f64 - t2581 - 0.19318136643975017455e-1_f64 * t2583 * t899 + 0.24147670804968771818e-2_f64 * t2588 + 0.18110753103726578864e-2_f64 * t893 * t2591 + 0.30184588506210964773e-2_f64 * t893 * t2598 - t862 * t2603 / 144.0_f64 + 0.35500316489081544176e-1_f64 * t874 * t2609 + 0.9176114905888133291e-1_f64 * t2613 * t314 - 0.19318136643975017455e-1_f64 * t2616;
    (t2607, t2608, t2609, t2612, t2613, t2618)
}
