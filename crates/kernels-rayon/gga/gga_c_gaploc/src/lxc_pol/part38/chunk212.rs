//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 212/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk212(t1029: f64, t531: f64, t1022: f64, t808: f64, t568: f64, t836: f64, t1036: f64, t317: f64, t797: f64, t813: f64, t833: f64, t960: f64, t971: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1040 = t531 * t1029;
    let t1043 = t808 * t1022;
    let t1044 = t568 * t1043;
    let t1048 = t836 * t1022;
    let t1049 = t568 * t1048;
    let t1052 = 0.35750489951850426669e0_f64 * t1036 * t317 + 0.29792074959875355558e-1_f64 * t960 - 0.35750489951850426669e0_f64 * t797 * t1040 - 0.23005755572352449806e1_f64 * t813 * t1044 - 0.19171462976960374838e0_f64 * t971 + 0.23005755572352449806e1_f64 * t833 * t1049;
    (t1040, t1043, t1044, t1048, t1049, t1052)
}
