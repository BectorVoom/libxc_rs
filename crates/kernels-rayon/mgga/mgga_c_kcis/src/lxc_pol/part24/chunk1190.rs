//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1190/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1190(t1709: f64, t330: f64, t14430: f64, t9985: f64, t1130: f64, t2178: f64, t26685: f64, t95781: f64, t26728: f64, t27856: f64, t3245: f64, t8054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95915 = t1709 * t330;
    let t95921 = t14430 * t9985;
    let t95926 = t2178 * t1130;
    let t95938 = 0.20612155671296296296e-4_f64 * t26685 * t95781;
    let t95963 = t26728 * t27856;
    let t96000 = t3245 * t8054;
    (t95915, t95921, t95926, t95938, t95963, t96000)
}
