//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 491/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk491(t397: f64, t4889: f64, t662: f64, t656: f64, t1774: f64, t25: f64) -> (f64, f64, f64) {
    let t4995 = t397 * t4889 * t662;
    let t4997 = 0.11993859144118211475e-1_f64 * t656 * t4995;
    let t4998 = t25 * t1774;
    (t4995, t4997, t4998)
}
