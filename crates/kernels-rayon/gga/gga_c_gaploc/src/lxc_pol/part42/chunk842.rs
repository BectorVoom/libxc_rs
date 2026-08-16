//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 842/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk842(t2508: f64, t37060: f64, t948: f64, t13535: f64, t7137: f64, t13507: f64, t7129: f64, t2717: f64, t3603: f64, t13556: f64, t35709: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44936 = 0.23071578690426672851e-1_f64 * t2508 * t37060 * t948;
    let t44938 = 0.10254034973522965712e-1_f64 * t7137 * t13535;
    let t44956 = 0.61524209841137794268e-1_f64 * t7137 * t13507;
    let t44960 = 0.76905262301422242837e-2_f64 * t7129 * t13535;
    let t44963 = 0.76905262301422242837e-2_f64 * t2508 * t2717 * t3603;
    let t44972 = 0.15381052460284448567e-1_f64 * t7129 * t13556;
    let t44973 = t35709 * t935;
    (t44936, t44938, t44956, t44960, t44963, t44972, t44973)
}
