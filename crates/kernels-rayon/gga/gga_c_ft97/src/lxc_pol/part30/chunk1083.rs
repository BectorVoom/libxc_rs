//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1083/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1083(t1882: f64, t35714: f64, t35649: f64, t35606: f64, t110669: f64, t111089: f64, t11593: f64, t1175: f64, t142296: f64, t142326: f64, t1449: f64, t150014: f64, t151081: f64, t242: f64, t2469: f64, t2574: f64, t2606: f64, t27742: f64, t28204: f64, t28208: f64, t28340: f64, t28345: f64, t28387: f64, t33274: f64, t33346: f64, t33754: f64, t33766: f64, t33782: f64, t35724: f64, t3746: f64, t3842: f64, t3972: f64, t3977: f64, t446: f64, t53797: f64, t54032: f64, t729: f64, t7440: f64, t762: f64, t98123: f64) -> f64 {
    let t152285 = t1882 * t35714;
    let t152328 = t1882 * t35649;
    let t152334 = t1882 * t35606;
    let t152347 = 2.0_f64 / 9.0_f64 * t152285 - 2.0_f64 / 9.0_f64 * t11593 * t2606 * t33754 * t3746 + 2.0_f64 / 3.0_f64 * t446 * t2574 * t1175 * t33346 + t446 * t729 * t33274 * t3842 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t242 * t151081 + t446 * t729 * t3977 * t33766 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t53797 * t110669 * t28208 + 4.0_f64 / 9.0_f64 * t53797 * t111089 * t28387 - 4.0_f64 / 27.0_f64 * t54032 * t111089 * t28345 + 4.0_f64 / 9.0_f64 * t53797 * t98123 * t28204 - t142296 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t729 * t2469 * t35724 + 2.0_f64 / 3.0_f64 * t446 * t729 * t762 * t27742 * t1449 - 4.0_f64 / 9.0_f64 * t152328 + 2.0_f64 / 3.0_f64 * t446 * t729 * t3977 * t33782 + 2.0_f64 / 3.0_f64 * t152334 - 2.0_f64 / 3.0_f64 * t446 * t2574 * t762 * t7440 * t3972 + t142326 + 2.0_f64 / 3.0_f64 * t446 * t242 * t150014 + 4.0_f64 / 9.0_f64 * t53797 * t110669 * t28340;
    t152347
}
