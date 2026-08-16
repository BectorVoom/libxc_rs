//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1276/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1276(t11678: f64, t11697: f64, t22279: f64, t22161: f64, t3577: f64, t19025: f64, t5001: f64, t22243: f64, t486: f64, t1222: f64, t22116: f64, t18332: f64, t4889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t72936 = t11678 * t11697 * t22279;
    let t72959 = t3577 * t11697 * t22161;
    let t72967 = t5001 * t19025;
    let t73028 = t486 * t22243;
    let t73043 = t22116 * t1222;
    let t73076 = t4889 * t18332;
    (t72936, t72959, t72967, t73028, t73043, t73076)
}
