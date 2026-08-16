//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 722/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk722(t1055: f64, t6815: f64, t1052: f64, t1066: f64, t1920: f64, t1923: f64, t1956: f64, t3026: f64, t3169: f64, t388: f64, t6680: f64, t6685: f64, t6687: f64, t6692: f64, t6695: f64, t6700: f64, t6707: f64, t6710: f64, t6769: f64, t6771: f64, t6776: f64) -> (f64, f64) {
    let t6816 = t1055 * t6815;
    let t6818 = -0.21932454224643019153e-1_f64 * t6680 * t1923 + t6685 + 0.27415567780803773942e-2_f64 * t6687 * t6692 - 0.82246703342411321825e-2_f64 * t6687 * t6695 + 0.82246703342411321825e-2_f64 * t1920 * t6700 - 0.82246703342411321825e-2_f64 * t6687 * t6707 + t6710 * t388 + t6769 * t388 - t6771 * t1066 - t3026 * t1956 - t3169 * t1956 + 2.0_f64 * t1052 * t6776 - t1052 * t6816;
    (t6816, t6818)
}
