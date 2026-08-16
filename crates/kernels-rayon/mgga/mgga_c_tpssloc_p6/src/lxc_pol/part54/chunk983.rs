//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 983/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk983(t23042: f64, t23063: f64, t23070: f64, t23084: f64, t25065: f64, t25069: f64, t25071: f64, t25073: f64, t25077: f64, t25080: f64, t25103: f64, t25107: f64, t25109: f64, t25113: f64, t25117: f64, t25121: f64, t25124: f64, t25126: f64, t25128: f64, t25133: f64, t25136: f64, t25158: f64) -> f64 {
    let t25160 = 0.20186378047070195427e-3_f64 * t25065 + 7.0_f64 / 2304.0_f64 * t23042 - t25069 / 384.0_f64 - t25071 / 384.0_f64 - t25073 / 384.0_f64 + 0.84782787797694820794e-2_f64 * t23063 + 7.0_f64 / 144.0_f64 * t23070 + 7.0_f64 / 576.0_f64 * t25077 + 0.14130464632949136799e-2_f64 * t23084 - 7.0_f64 / 2304.0_f64 * t25080 + t25103 - 0.12111826828242117256e-2_f64 * t25107 + 0.84782787797694820792e-2_f64 * t25109 + 0.12111826828242117256e-2_f64 * t25113 - 0.20186378047070195427e-3_f64 * t25117 + 0.84782787797694820792e-2_f64 * t25121 - 0.20186378047070195427e-3_f64 * t25124 + 0.14130464632949136799e-2_f64 * t25126 - t25128 / 48.0_f64 + 0.33643963411783659045e-4_f64 * t25133 + t25136 / 1536.0_f64 + t25158;
    t25160
}
