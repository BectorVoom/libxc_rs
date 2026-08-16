//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 563/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk563(t1450: f64, t7831: f64, t1340: f64, t1411: f64, t2232: f64, t5886: f64, t2236: f64, t5606: f64, t3530: f64, t3533: f64, t7706: f64, t2075: f64, t2083: f64, t3539: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7832 = t1450 * t7831;
    let t7833 = t1340 * t7832;
    let t7834 = t1411 * t7833;
    let t7836 = t5886 * t2232;
    let t7837 = t1411 * t7836;
    let t7839 = t5606 * t2236;
    let t7840 = t1411 * t7839;
    let t7846 = t3530 * t3533 * t7706;
    let t7850 = t3539 * t2075 * t2083;
    (t7832, t7833, t7834, t7836, t7837, t7839, t7840, t7846, t7850)
}
