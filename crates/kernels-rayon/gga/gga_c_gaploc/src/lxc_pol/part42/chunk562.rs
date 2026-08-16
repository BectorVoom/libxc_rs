//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 562/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk562(t10811: f64, t2631: f64, t2628: f64, t2976: f64, t1022: f64, t7284: f64, t787: f64, t2639: f64, t10627: f64, t723: f64) -> (f64, f64, f64, f64, f64) {
    let t10813 = 0.42900587942220512003e1_f64 * t10811 * t2631;
    let t10814 = t2976 * t2628;
    let t10815 = 0.29792074959875355558e-1_f64 * t10814;
    let t10816 = t7284 * t1022;
    let t10817 = t787 * t10816;
    let t10819 = 0.25025342966295298669e1_f64 * t10817 * t2639;
    let t10820 = t10627 * t723;
    (t10813, t10814, t10815, t10819, t10820)
}
