//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2097/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2097(t27495: f64, t85964: f64, t15702: f64, t8038: f64, t85822: f64, t27563: f64, t85639: f64, t24826: f64, t27502: f64, t27558: f64, t7368: f64, t94490: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94874 = t85964 * t27495;
    let t94881 = t85822 * t8038 * t15702;
    let t94885 = 0.36554090374405031922e-2_f64 * t85639 * t27563;
    let t94889 = 0.54831135561607547884e-2_f64 * t24826 * t27502;
    let t94891 = 0.18277045187202515961e-2_f64 * t85639 * t27558;
    let t94901 = 0.14621636149762012769e-1_f64 * t94490 * t7368;
    (t94874, t94881, t94885, t94889, t94891, t94901)
}
