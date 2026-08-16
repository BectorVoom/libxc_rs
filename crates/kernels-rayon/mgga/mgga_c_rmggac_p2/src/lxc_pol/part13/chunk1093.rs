//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1093/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1093(t41716: f64, t41722: f64, t41725: f64, t4961: f64, t702: f64, t1668: f64, t2265: f64, t289: f64, t36590: f64, t36594: f64, t37950: f64, t41701: f64, t41706: f64, t41713: f64, t41719: f64, t41727: f64, t41730: f64, t41735: f64, t41739: f64, t530: f64, t5355: f64, t8048: f64, t931: f64, t9343: f64) -> f64 {
    let t43810 = 0.19158786722982093702e1_f64 * t41716;
    let t43812 = 0.3193131120497015617e0_f64 * t41722;
    let t43813 = 0.95793933614910468512e0_f64 * t41725;
    let t43817 = t4961 * t702;
    let t43827 = -0.4726e1_f64 * t530 * t37950 - 0.1276937996798935182e-3_f64 * t41701 - 0.5107751987195740728e-4_f64 * t41706 + 0.36366215538993788974e-1_f64 * t36590 + 0.18183107769496894487e-1_f64 * t36594 + 0.17961362552795712846e1_f64 * t41713 + t43810 - 0.11974241701863808564e0_f64 * t41719 - t43812 - t43813 + 0.66671395154821946452e-1_f64 * t41727 - 0.2363e1_f64 * t931 * t9343 - 0.4726e1_f64 * t289 * t43817 - 0.85129199786595678799e-5_f64 * t41730 - 0.72732431077987577947e-1_f64 * t41735 - 0.40911992481368012595e-1_f64 * t41739 - 0.2363e1_f64 * t5355 * t2265 - 0.4726e1_f64 * t1668 * t8048;
    t43827
}
