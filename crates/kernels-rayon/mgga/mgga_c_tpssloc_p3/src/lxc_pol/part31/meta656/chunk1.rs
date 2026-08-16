//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1939/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1939(t5587: f64, t81803: f64, t1512: f64, t87295: f64, t23097: f64, t232: f64, t67793: f64, t815: f64, t2628: f64, t5585: f64, t776: f64, t13228: f64, t4233: f64, t6605: f64) -> (f64, f64, f64, f64, f64) {
    let t98752 = t81803 * t5587;
    let t98754 = t87295 * t1512;
    let t98758 = t23097 * t815 * t67793 * t232;
    let t98762 = t23097 * t2628 * t5585 * t776;
    let t98766 = t6605 * t2628 * t13228 * t4233;
    (t98752, t98754, t98758, t98762, t98766)
}
