//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1063/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1063(t1466: f64, t5997: f64, t11825: f64, t4291: f64, t12534: f64, t251: f64, t1532: f64, t1929: f64, t2060: f64, t577: f64, t1467: f64, t12520: f64, t492: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17449 = t5997 * t1466;
    let t17450 = t17449 * sigma2;
    let t17463 = t11825 * t4291;
    let t17470 = t251 * t12534;
    let t17474 = t1532 * t1929;
    let t17504 = t577 * t2060;
    let t17505 = t1467 * t17504;
    let t17508 = t12520 * t492;
    (t17450, t17463, t17470, t17474, t17504, t17505, t17508)
}
