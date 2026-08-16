//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 924/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk924(t2825: f64, t6508: f64, t2842: f64, t19159: f64, t4546: f64, t3202: f64, t3200: f64, t6696: f64, t922: f64, t1121: f64, t6613: f64, t1022: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19778 = t2825 * t6508;
    let t19779 = t2842 * t19778;
    let t19781 = t4546 * t19159;
    let t19782 = t3202 * t19781;
    let t19783 = t3200 * t19782;
    let t19785 = t6696 * t922;
    let t19786 = t3202 * t19785;
    let t19787 = t3200 * t19786;
    let t19789 = t6613 * t1121;
    let t19790 = t1022 * t19789;
    (t19779, t19781, t19783, t19785, t19787, t19789, t19790)
}
