//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1050/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1050(t1466: f64, t5869: f64, t12274: f64, t2013: f64, t3728: f64, t5761: f64, t4158: f64, t4992: f64, t86: f64, t5659: f64, t11913: f64, t5668: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16751 = t5869 * t1466;
    let t16752 = t16751 * sigma2;
    let t16756 = t12274 * t2013;
    let t16768 = t3728 * t5761;
    let t16769 = 0.22109259259259259258e-2_f64 * t16768;
    let t16771 = t86 * t4992 * t4158;
    let t16788 = t86 * t4992 * t5659;
    let t16793 = t11913 * t5668;
    (t16751, t16752, t16756, t16768, t16769, t16771, t16788, t16793)
}
