//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1123/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1123(t330: f64, t4625: f64, t829: f64, t3269: f64, t2635: f64, t4595: f64, t1670: f64, t2844: f64, t10292: f64, t2630: f64, t313: f64, t4600: f64) -> (f64, f64, f64, f64) {
    let t14182 = t4625 * t330;
    let t14183 = t14182 * t829;
    let t14184 = t3269 * t14183;
    let t14188 = t3269 * t4595 * t2635;
    let t14191 = t1670 * t2844;
    let t14193 = t10292 * t14191 * t2630;
    let t14196 = t4600 * t313;
    (t14184, t14188, t14193, t14196)
}
