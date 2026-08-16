//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3257/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3257<F: Float>(t125: F, t18615: F, t10744: F, t18418: F, t808: F, t10900: F, t18627: F, t2394: F, t2724: F, t2747: F, t4362: F, t4364: F, t4366: F, t50573: F, t50577: F, t50579: F, t50581: F, t50586: F, t50590: F, t50594: F, t50598: F, t5984: F, t800: F) -> (F, F) {
    let t61791 = t125 * t18615;
    let t61797 = t10744 * t808 * t18418;
    let t61814 = -F::cast_from(0.15246000842785598467e-3_f64) * t50573 + F::cast_from(0.85748036236139473944e-3_f64) * t4362 * t4364 * t61791 * t4366 + F::cast_from(0.25410001404642664112e-5_f64) * t61797 - t10900 * t800 * t5984 * t2394 / F::cast_from(4.0_f64) - F::cast_from(0.57165357490759649296e-4_f64) * t50577 + F::cast_from(0.20007875121765877254e-2_f64) * t50579 + F::cast_from(0.54208002996571016772e-3_f64) * t50581 - F::cast_from(0.11433071498151929859e-3_f64) * t50586 - F::cast_from(0.57165357490759649296e-4_f64) * t50590 - F::cast_from(0.17149607247227894789e-2_f64) * t4362 * t2747 * t18627 * t2724 + F::cast_from(0.10164000561857065645e-3_f64) * t50594 + F::cast_from(0.15246000842785598467e-3_f64) * t50598;
    (t61791, t61814)
}
