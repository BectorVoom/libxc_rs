//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1625/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1625(t1246: f64, t19128: f64, t5079: f64, t6256: f64, t3625: f64, t5011: f64, t1755: f64, t5068: f64, t1235: f64, t6224: f64, t1215: f64, t475: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19129 = t19128 * t1246;
    let t19131 = t6256 * t5079;
    let t19138 = t3625 * t5011;
    let t19139 = t1755 * t19138;
    let t19142 = t6256 * t5068;
    let t19145 = t1235 * t6224;
    let t19146 = t19145 * t3625;
    let t19153 = t6739 * t1215 * t475;
    (t19129, t19131, t19138, t19139, t19142, t19145, t19146, t19153)
}
