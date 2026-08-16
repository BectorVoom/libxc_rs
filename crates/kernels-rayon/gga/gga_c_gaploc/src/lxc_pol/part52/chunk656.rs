//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 656/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk656(t1589: f64, t3726: f64, t10850: f64, t10853: f64, t10855: f64, t10859: f64, t12163: f64, t12167: f64, t12173: f64, t12177: f64, t12182: f64, t12185: f64, t2049: f64, t2194: f64, t2197: f64, t317: f64, t3733: f64, t3736: f64, t3741: f64, t3746: f64, t784: f64, t797: f64, t813: f64, t833: f64) -> f64 {
    let t12188 = t1589 * t3726;
    let t12191 = 0.23833659967900284446e0_f64 * t3733 * t784 + 0.23005755572352449806e1_f64 * t2197 * t3746 + 0.23005755572352449806e1_f64 * t833 * t12163 - 0.35750489951850426669e0_f64 * t797 * t12167 - 0.23005755572352449806e1_f64 * t2194 * t3741 - 0.23005755572352449806e1_f64 * t813 * t12173 + 0.35750489951850426669e0_f64 * t12177 * t317 - 0.35750489951850426669e0_f64 * t2049 * t3736 + 0.35750489951850426669e0_f64 * t12182 * t317 - 0.30674340763136599741e1_f64 * t813 * t12185 - 0.23833659967900284446e0_f64 * t797 * t12188 - t10850 + t10853 - t10855 - t10859;
    t12191
}
