//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 555/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk555(t3626: f64, t5668: f64, t7738: f64, t7742: f64, t7746: f64, t321: f64, t2093: f64, t5715: f64, t2092: f64, t1191: f64, t3639: f64, t2083: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7748 = t3626 + 0.11872222222222222222e-1_f64 * t5668 - 0.11872222222222222222e-1_f64 * t7738 + 0.35616666666666666666e-1_f64 * t7742 - 0.17808333333333333333e-1_f64 * t7746;
    let t7750 = 0.62182e-1_f64 * t7748 * t321;
    let t7752 = 2.0_f64 * t5715 * t2093;
    let t7753 = t2092 * t2092;
    let t7754 = t7753 * t1191;
    let t7756 = 2.0_f64 * t3639 * t7754;
    let t7757 = t2083 * t2083;
    (t7748, t7750, t7752, t7753, t7754, t7756, t7757)
}
