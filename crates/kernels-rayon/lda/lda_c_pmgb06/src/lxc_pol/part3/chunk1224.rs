//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1224/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1224(t117: f64, t123: f64, t2360: f64, t740: f64, t10795: f64, t10799: f64, t10802: f64, t10806: f64, t10808: f64, t10811: f64, t10813: f64, t10817: f64, t10820: f64, t10823: f64, t10825: f64, t10828: f64, t10831: f64, t10834: f64, t10838: f64) -> f64 {
    let t14500 = t123 * t740 * t2360 * t117;
    let t14501 = 0.07184540406152766_f64 * t14500;
    let t14511 = -t14501 + 0.010403978958430045_f64 * t10795 - 0.0014862827083471494_f64 * t10799 - 0.004458848125041448_f64 * t10802 - t10806 - t10808 - t10811 - 0.01777850129601853_f64 * t10813 + t10817 - 0.001975389032890948_f64 * t10820 - 0.01185233419734569_f64 * t10823 - 0.07769863529371063_f64 * t10825 - t10828 + 0.01975389032890948_f64 * t10831 + 0.059261670986728444_f64 * t10834 + t10838;
    t14511
}
