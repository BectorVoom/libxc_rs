//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 656/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk656<F: Float>(t1589: F, t3726: F, t10850: F, t10853: F, t10855: F, t10859: F, t12163: F, t12167: F, t12173: F, t12177: F, t12182: F, t12185: F, t2049: F, t2194: F, t2197: F, t317: F, t3733: F, t3736: F, t3741: F, t3746: F, t784: F, t797: F, t813: F, t833: F) -> F {
    let t12188 = t1589 * t3726;
    let t12191 = F::cast_from(0.23833659967900284446e0_f64) * t3733 * t784 + F::cast_from(0.23005755572352449806e1_f64) * t2197 * t3746 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t12163 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t12167 - F::cast_from(0.23005755572352449806e1_f64) * t2194 * t3741 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t12173 + F::cast_from(0.35750489951850426669e0_f64) * t12177 * t317 - F::cast_from(0.35750489951850426669e0_f64) * t2049 * t3736 + F::cast_from(0.35750489951850426669e0_f64) * t12182 * t317 - F::cast_from(0.30674340763136599741e1_f64) * t813 * t12185 - F::cast_from(0.23833659967900284446e0_f64) * t797 * t12188 - t10850 + t10853 - t10855 - t10859;
    t12191
}
