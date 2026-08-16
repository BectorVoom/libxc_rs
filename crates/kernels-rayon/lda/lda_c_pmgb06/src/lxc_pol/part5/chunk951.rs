//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 951/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk951(t117: f64, t123: f64, t2360: f64, t740: f64, t1147: f64, t859: f64, t2791: f64, t795: f64, t1347: f64, t1799: f64, t1795: f64, t118: f64, t5575: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14500 = t123 * t740 * t2360 * t117;
    let t14501 = 0.07184540406152766_f64 * t14500;
    let t14527 = t123 * t1147 * t859 * t117;
    let t14529 = t795 * t2791;
    let t14535 = t1799 * t1347;
    let t14536 = 0.09451622166942335_f64 * t14535;
    let t14541 = t1795 * t1347;
    let t14543 = t5575 * t118;
    (t14501, t14527, t14529, t14536, t14541, t14543)
}
