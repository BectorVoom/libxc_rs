//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 899/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk899(t21114: f64, t932: f64, t1557: f64, t17195: f64, t4354: f64, t5727: f64, t13520: f64, t5730: f64, t21252: f64, t2844: f64, t10661: f64, t10675: f64, t10676: f64, t21120: f64, t21124: f64, t21128: f64, t21132: f64, t21136: f64, t21140: f64, t21142: f64, t21144: f64, t21147: f64, t21150: f64, t21153: f64, t21156: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21259 = t21114 * t932;
    let t21263 = 3.0_f64 * t17195 * t1557;
    let t21265 = 3.0_f64 * t4354 * t5727;
    let t21267 = 0.48245938496077605201e2_f64 * t13520 * t5730;
    let t21268 = t21252 * t2844;
    let t21270 = 0.96491876992155210402e2_f64 * t10661 * t21268;
    let t21283 = 0.16431333333333333333e0_f64 * t21120 - 0.59793333333333333333e0_f64 * t21124 + 0.17938e1_f64 * t21128 - 0.36514074074074074075e-1_f64 * t21132 - 0.82156666666666666667e-1_f64 * t21136 - 0.49293999999999999999e0_f64 * t21140 - 0.28483875e1_f64 * t21142 + 0.46074375e0_f64 * t21144 - t10675 - t10676 - 0.33218518518518518518e0_f64 * t21147 + 0.11958666666666666667e1_f64 * t21150 - 0.17938e1_f64 * t21153 - 0.29896666666666666667e0_f64 * t21156;
    (t21259, t21263, t21265, t21267, t21270, t21283)
}
