//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1453/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1453(t109461: f64, t109493: f64, t109528: f64, t109553: f64, t109593: f64, t109627: f64, t109661: f64, t109694: f64, t109535: f64, t1887: f64, t103515: f64, t103694: f64, t103881: f64, t104469: f64, t104480: f64, t109418: f64, t1653: f64, t1729: f64, t1735: f64, t2149: f64, t2152: f64, t21762: f64, t22114: f64, t24776: f64, t24812: f64, t24821: f64, t27406: f64, t27496: f64, t29678: f64, t29763: f64, t29773: f64, t3610: f64, t3612: f64, t470: f64, t493: f64, t7283: f64, t7362: f64, t7363: f64, t8078: f64, t86037: f64, t95768: f64) -> (f64, f64, f64) {
    let t109697 = t109461 + t109493 + t109528 + t109553 + t109593 + t109627 + t109661 + t109694;
    let t109722 = t109535 * t1887;
    let t109732 = t470 * t493 * t109697 + 3.0_f64 * t1729 * t29773 + t22114 * t2152 + 0.14621636149762012769e-1_f64 * t95768 + 0.43864908449286038307e-1_f64 * t27406 * t29763 + 0.21932454224643019154e-1_f64 * t7283 * t24776 * t7363 * t21762 + 0.54831135561607547883e-2_f64 * t104469 - 0.82246703342411321826e-2_f64 * t7283 * t7362 * t103881 * t1653 + 6.0_f64 * t3610 * t109418 * t3612 + 0.82246703342411321826e-2_f64 * t86037 * t103694 * t24821 * t1653 - 0.3752886611772249944e0_f64 * t109722 * t2149 + 0.24125699647107321069e0_f64 * t29678 * t8078 - 0.24674011002723396548e-1_f64 * t24812 * t27496 * t103515 * t1735 + 0.82246703342411321826e-2_f64 * t104480;
    (t109697, t109722, t109732)
}
