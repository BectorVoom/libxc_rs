//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 792/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk792(t2005: f64, t5483: f64, t1849: f64, t2020: f64, t2023: f64, t3290: f64, t1775: f64, t1060: f64, t5515: f64, t5491: f64, t10791: f64, t397: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t12230 = t2005 * t5483;
    let t12234 = t2020 * t1849;
    let t12235 = t3290 * t2023;
    let t12236 = t12234 * t12235;
    let t12237 = t1775 * t12236;
    let t12240 = t1060 * t5515;
    let t12241 = t5491 * t12240;
    let t12242 = t1775 * t12241;
    let t12246 = t397 * t10791 * t786;
    (t12230, t12235, t12237, t12242, t12246)
}
