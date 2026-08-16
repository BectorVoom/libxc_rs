//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 581/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk581(t10746: f64, t10716: f64, t10720: f64, t10722: f64, t10733: f64, t10735: f64, t10739: f64, t10741: f64, t10745: f64, t1897: f64, t3434: f64, t3452: f64, t650: f64, t681: f64, t9754: f64, t9762: f64) -> f64 {
    let t10747 = 0.32043859292259267849e-3_f64 * t10746;
    let t10748 = t10716 + t10720 + 0.76905262301422242837e-2_f64 * t1897 * t10722 - 0.76905262301422242837e-2_f64 * t681 * t3452 + 0.10254034973522965712e-1_f64 * t650 * t3434 - 0.10254034973522965712e-1_f64 * t650 * t3452 + t9754 + t9762 + t10733 + t10735 - t10739 - t10741 - t10745 + t10747;
    t10748
}
