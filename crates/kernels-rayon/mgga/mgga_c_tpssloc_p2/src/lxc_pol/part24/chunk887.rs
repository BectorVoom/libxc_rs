//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 887/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk887(t10103: f64, t858: f64, t856: f64, t68: f64, t2719: f64, t865: f64, t2742: f64, t2718: f64, t10047: f64, t10049: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t2743: f64, t855: f64, t866: f64, t9520: f64, t9585: f64, t9587: f64, t9590: f64, t9593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10104 = t858 * t10103;
    let t10108 = t856 * t856;
    let t10109 = 1.0_f64 / t10108;
    let t10110 = t68 * t10109;
    let t10111 = t2719 * t865;
    let t10112 = t10110 * t10111;
    let t10115 = t865 * t2742;
    let t10116 = t2718 * t10115;
    let t10121 = t10047 * t259 - 3.0_f64 * t10049 * t866 - t10104 * t855 - 6.0_f64 * t10112 * t855 + 6.0_f64 * t10116 * t855 + 3.0_f64 * t259 * t9520 + t259 * t9585 + 3.0_f64 * t259 * t9587 + 6.0_f64 * t2597 * t2720 - 3.0_f64 * t2597 * t2743 + 6.0_f64 * t2713 * t2720 - 3.0_f64 * t2713 * t2743 - 3.0_f64 * t866 * t9590 - 6.0_f64 * t866 * t9593;
    (t10104, t10108, t10109, t10110, t10111, t10112, t10115, t10116, t10121)
}
