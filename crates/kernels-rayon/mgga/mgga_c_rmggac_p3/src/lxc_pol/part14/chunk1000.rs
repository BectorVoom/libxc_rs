//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1000/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1000(t2123: f64, t570: f64, t321: f64, t352: f64, t35862: f64, t35865: f64, t35869: f64, t41029: f64, t41033: f64, t41037: f64, t41042: f64, t41045: f64, t41049: f64, t41053: f64, t41057: f64, t41059: f64, t5259: f64, t8940: f64) -> (f64, f64) {
    let t41063 = t2123 * t570;
    let t41069 = -0.10000709273223291967e0_f64 * t41029 + 0.13334279030964389289e0_f64 * t41033 - 0.72732431077987577942e-1_f64 * t41037 - t41042 - 0.10227998120342003148e-1_f64 * t41045 + 0.13637330827122670864e-1_f64 * t41049 + 0.34093327067806677161e-2_f64 * t41053 + 0.54549323308490683456e-1_f64 * t41057 + 0.23948483403727617128e0_f64 * t5259 * t41059 * t321 + 0.23948483403727617128e0_f64 * t8940 * t41063 * t352 - t35862 - 0.36366215538993788972e-1_f64 * t35865 - 0.90915538847484472429e-2_f64 * t35869;
    (t41063, t41069)
}
