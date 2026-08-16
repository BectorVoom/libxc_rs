//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1194/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1194(t34995: f64, t3801: f64, t125070: f64, t125074: f64, t125092: f64, t1298: f64, t1300: f64, t131426: f64, t131474: f64, t131512: f64, t131552: f64, t131599: f64, t131640: f64, t131686: f64, t131725: f64, t131771: f64, t131815: f64, t131849: f64, t131882: f64, t131925: f64, t131966: f64, t132005: f64, t132047: f64, t1832: f64, t198: f64, t27037: f64, t27041: f64, t29313: f64, t29322: f64, t33533: f64, t33539: f64, t336: f64, t5023: f64, t5501: f64, t7669: f64, t7673: f64, t8220: f64) -> f64 {
    let t132055 = t34995 * t3801;
    let t132085 = t198 * t336 * (t131426 + t131474 + t131512 + t131552 + t131599 + t131640 + t131686 + t131725 + t131771 + t131815 + t131849 + t131882 + t131925 + t131966 + t132005 + t132047) * t1300 - t5023 * t132055 * t1298 - t5023 * t125070 * t1832 + 2.0_f64 * t5023 * t125074 * t29322 - t5023 * t33533 * t5501 - 2.0_f64 * t5023 * t27037 * t8220 + 4.0_f64 * t5023 * t27041 * t8220 * t1298 - 2.0_f64 * t5023 * t7673 * t29313 + 4.0_f64 * t5023 * t27041 * t1832 * t7669 - 6.0_f64 * t5023 * t125092 * t29322 + 2.0_f64 * t5023 * t33539 * t5501;
    t132085
}
