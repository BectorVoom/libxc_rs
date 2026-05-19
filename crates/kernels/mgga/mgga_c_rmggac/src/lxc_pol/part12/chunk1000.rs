//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1000/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1000<F: Float>(t2123: F, t570: F, t321: F, t352: F, t35862: F, t35865: F, t35869: F, t41029: F, t41033: F, t41037: F, t41042: F, t41045: F, t41049: F, t41053: F, t41057: F, t41059: F, t5259: F, t8940: F) -> (F, F) {
    let t41063 = t2123 * t570;
    let t41069 = -F::cast_from(0.10000709273223291967e0_f64) * t41029 + F::cast_from(0.13334279030964389289e0_f64) * t41033 - F::cast_from(0.72732431077987577942e-1_f64) * t41037 - t41042 - F::cast_from(0.10227998120342003148e-1_f64) * t41045 + F::cast_from(0.13637330827122670864e-1_f64) * t41049 + F::cast_from(0.34093327067806677161e-2_f64) * t41053 + F::cast_from(0.54549323308490683456e-1_f64) * t41057 + F::cast_from(0.23948483403727617128e0_f64) * t5259 * t41059 * t321 + F::cast_from(0.23948483403727617128e0_f64) * t8940 * t41063 * t352 - t35862 - F::cast_from(0.36366215538993788972e-1_f64) * t35865 - F::cast_from(0.90915538847484472429e-2_f64) * t35869;
    (t41063, t41069)
}
