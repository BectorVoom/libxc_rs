//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1356/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1356(t10110: f64, t1912: f64, t22979: f64, t23190: f64, t23215: f64, t23281: f64, t2713: f64, t2718: f64, t2719: f64, t2743: f64, t40852: f64, t40870: f64, t6632: f64, t6662: f64, t6663: f64, t81554: f64, t81559: f64, t81621: f64, t81683: f64, t82000: f64, t82060: f64, t82070: f64, t82071: f64, t82076: f64, t82079: f64, t82082: f64, t82117: f64, t82149: f64, t82186: f64, t82197: f64, t82209: f64, t82211: f64, t82246: f64, t82279: f64, t82304: f64, t855: f64, t858: f64, t865: f64, t866: f64, t9590: f64, t9593: f64) -> f64 {
    let t82307 = -18.0_f64 * t2713 * t23215 + 12.0_f64 * t2713 * t22979 - 6.0_f64 * t9593 * t6663 - 3.0_f64 * t40870 * t1912 - 3.0_f64 * t82071 * t866 - 3.0_f64 * t82197 * t866 - t40852 * t1912 + 6.0_f64 * t9590 * t6632 - 3.0_f64 * t23281 * t2743 + 0.82246703342411321825e-2_f64 * t81554 + t82304 + t82279 + t82246 - 0.38381794893125283518e0_f64 * t82209 - 0.19190897446562641759e0_f64 * t82211 + t82186 + t82149 + t82117 + 0.24674011002723396547e-1_f64 * t82082 - 0.49348022005446793095e-1_f64 * t82076 + 0.12337005501361698274e-1_f64 * t82079 + t82070 + 0.49348022005446793095e-1_f64 * t81559 - t855 * t858 * (t81621 + t81683 + t82000 + t82060) - 18.0_f64 * t855 * t10110 * t6662 * t2719 + 6.0_f64 * t855 * t2718 * t23190 * t865;
    t82307
}
