//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 915/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk915<F: Float>(t18023: F, t3092: F, t914: F, t1150: F, t1162: F, t12578: F, t12581: F, t17994: F, t18006: F, t18009: F, t18013: F, t18016: F, t18020: F, t3103: F, t3234: F, t4435: F, t4492: F, t4501: F, t5389: F, t5394: F, t5399: F, t5404: F) -> (F, F, F) {
    let t18024 = t3092 * t18023;
    let t18025 = t914 * t18024;
    let t18028 = 0.11360101276506094136e1 * t1150 * t17994 - 0.90880810212048753088e1 * t4501 * t5404 + 0.35163949364965747848e4 * t12581 * t5399 - 0.70327898729931495696e4 * t12578 * t5394 - 0.9356877183176434872e2 * t4492 * t5389 + 0.4645868436449114021e2 * t4435 * t18006 + 0.1169609647897054359e2 * t3234 * t18009 + 0.1949349413161757265e2 * t3234 * t18013 - 0.15486228121497046737e2 * t3103 * t18016 + 0.1169609647897054359e2 * t3234 * t18020 - 0.17386322979577515709e0 * t1162 * t18025;
    (t18024, t18025, t18028)
}
