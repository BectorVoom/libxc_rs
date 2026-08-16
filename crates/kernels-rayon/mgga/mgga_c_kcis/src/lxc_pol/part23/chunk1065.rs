//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1065/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1065(t12581: f64, t2256: f64, t2260: f64, t27333: f64, t27335: f64, t27337: f64, t27362: f64, t27366: f64, t27556: f64, t27560: f64, t27564: f64, t27567: f64, t27569: f64, t7968: f64, t7971: f64, t7978: f64) -> (f64, f64) {
    let t27575 = t12581 * t2256;
    let t27582 = 0.92754700520833333334e-4_f64 * t27556 * t7971 + 0.46377350260416666667e-4_f64 * t7968 * t27560 + 0.30918233506944444444e-4_f64 * t27564 + 0.30918233506944444444e-4_f64 * t27567 * t27569 + 0.34822083333333333332e-2_f64 * t27333 - 0.23214722222222222222e-2_f64 * t27335 + 0.15476481481481481481e-2_f64 * t27337 - 0.34752604166666666667e-3_f64 * t27575 * t2260 + 0.34752604166666666667e-3_f64 * t7978 * t27560 + 0.15476481481481481481e-2_f64 * t27362 + 0.23214722222222222222e-2_f64 * t27366;
    (t27575, t27582)
}
