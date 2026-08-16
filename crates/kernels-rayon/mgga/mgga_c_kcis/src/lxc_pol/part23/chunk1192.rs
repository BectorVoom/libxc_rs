//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1192/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1192(t27543: f64, t3733: f64, t4244: f64, t491: f64, t12520: f64, t12564: f64, t4188: f64, t7938: f64, t12939: f64, t2264: f64, t4479: f64, t7996: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94748 = t3733 * t27543;
    let t94754 = t4244 * t491;
    let t94785 = t12520 * t491;
    let t94805 = t12564 * t491;
    let t94816 = t7938 * t4188;
    let t94819 = t2264 * t12939;
    let t94824 = t7996 * t4479;
    (t94748, t94754, t94785, t94805, t94816, t94819, t94824)
}
