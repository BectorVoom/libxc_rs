//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1273/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1273(t1888: f64, t232: f64, t6646: f64, t81672: f64, t2627: f64, t6624: f64, t10016: f64, t1909: f64, t22993: f64, t23009: f64, t2617: f64, t2633: f64, t812: f64, t81623: f64, t81627: f64, t81630: f64, t81633: f64, t81637: f64, t81642: f64, t81645: f64, t81648: f64, t81653: f64, t81656: f64, t81661: f64, t81667: f64, t81670: f64) -> f64 {
    let t81675 = t1888 * t6646 * t81672 * t232;
    let t81679 = t2627 * t6624;
    let t81683 = t10016 * t1909 + 0.23029076935875170111e0_f64 * t81623 - 0.16449340668482264365e-1_f64 * t81627 + 0.24674011002723396548e-1_f64 * t81630 - 0.38381794893125283518e0_f64 * t81633 - 0.49348022005446793095e-1_f64 * t81637 - 0.74022033008170189643e-1_f64 * t81642 + 0.49348022005446793095e-1_f64 * t81645 - 0.24674011002723396548e-1_f64 * t81648 - 0.49348022005446793095e-1_f64 * t81653 + 0.49348022005446793095e-1_f64 * t81656 - 0.49348022005446793095e-1_f64 * t81661 - 6.0_f64 * t2617 * t22993 - 0.24674011002723396548e-1_f64 * t81667 + 0.24674011002723396547e-1_f64 * t81670 - 0.82246703342411321825e-2_f64 * t81675 + 6.0_f64 * t2617 * t23009 + 6.0_f64 * t812 * t81679 * t2633;
    t81683
}
