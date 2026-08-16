//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1056/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1056(t25949: f64, t7063: f64, t3974: f64, t7259: f64, t2482: f64, t27: f64, t7269: f64, t3981: f64, t2019: f64, t3985: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25950 = t7063 * t25949;
    let t25969 = t7259 * t3974;
    let t25972 = t2482 * t7269 * t27;
    let t25973 = t25972 * t3981;
    let t25975 = t2019 * t3985;
    let t25978 = t820 * t7269 * t843;
    (t25950, t25969, t25972, t25973, t25975, t25978)
}
