//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1626/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1626(t19153: f64, t6252: f64, t11889: f64, t1215: f64, t5079: f64, t6260: f64, t11888: f64, t11904: f64, t11907: f64, t11914: f64, t1244: f64, t15027: f64, t15032: f64, t15245: f64, t1756: f64, t19123: f64, t19129: f64, t19131: f64, t19139: f64, t19142: f64, t19146: f64, t3604: f64, t3610: f64, t3624: f64, t5064: f64, t5069: f64, t5080: f64, t5084: f64, t6253: f64, t6261: f64, t6263: f64) -> (f64, f64) {
    let t19154 = t6252 * t19153;
    let t19156 = t11889 * t1215;
    let t19157 = t6252 * t19156;
    let t19160 = t6260 * t5079;
    let t19164 = -6.0_f64 * t11888 * t19157 + 2.0_f64 * t11904 * t6253 - t11907 * t6263 + t11914 * t19154 + t1244 * t19129 + 4.0_f64 * t15027 * t5069 + 2.0_f64 * t15032 * t1756 - 2.0_f64 * t15245 * t5080 + 2.0_f64 * t19123 * t3610 - 2.0_f64 * t19131 * t3624 - 2.0_f64 * t19139 * t3624 + 4.0_f64 * t19142 * t3610 - t19146 * t3624 - t19160 * t3624 + t3604 * t6261 + 2.0_f64 * t5064 * t5084;
    (t19156, t19164)
}
