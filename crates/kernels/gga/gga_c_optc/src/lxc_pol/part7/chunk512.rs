//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 512/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk512<F: Float>(t1: F, t2606: F, t297: F, t313: F, t2246: F, t301: F, t300: F, t885: F, t889: F, t2574: F, t2577: F, t2581: F, t2583: F, t2588: F, t2591: F, t2598: F, t2603: F, t289: F, t314: F, t862: F, t874: F, t893: F, t899: F) -> (F, F, F, F, F, F) {
    let t2607 = t2606 * t1;
    let t2608 = t2607 * t297;
    let t2609 = t313 * t2608;
    let t2612 = t301 * t2246;
    let t2613 = t300 * t2612;
    let t2616 = t885 * t889;
    let t2618 = 11.0 / 108.0 * t2574 * t289 - t2577 / 54.0 - t2581 - 0.19318136643975017455e-1 * t2583 * t899 + 0.24147670804968771818e-2 * t2588 + 0.18110753103726578864e-2 * t893 * t2591 + 0.30184588506210964773e-2 * t893 * t2598 - t862 * t2603 / 144.0 + 0.35500316489081544176e-1 * t874 * t2609 + 0.9176114905888133291e-1 * t2613 * t314 - 0.19318136643975017455e-1 * t2616;
    (t2607, t2608, t2609, t2612, t2613, t2618)
}
