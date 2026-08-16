//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 680/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk680(t793: f64, t9005: f64, t2064: f64, t558: f64, t797: f64, t5271: f64, t8625: f64, t5162: f64, t8631: f64, t4669: f64, t8635: f64, t8645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9006 = t793 * t9005;
    let t9008 = t2064 * t558;
    let t9009 = t797 * t9008;
    let t9011 = t5271 * t8625;
    let t9013 = t5162 * t8631;
    let t9015 = t4669 * t8635;
    let t9017 = t5271 * t8645;
    (t9006, t9008, t9009, t9011, t9013, t9015, t9017)
}
