//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3838/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3838(t46917: f64, t6871: f64, t22298: f64, t48862: f64, t48863: f64, t22098: f64, t9962: f64, t22102: f64, t46740: f64, t13783: f64, t13789: f64, t13790: f64, t13791: f64, t1398: f64, t22079: f64, t22118: f64, t36776: f64, t3934: f64, t3938: f64, t4004: f64, t48475: f64, t49146: f64, t5671: f64, t6816: f64, t6862: f64, t6869: f64, t9955: f64, t9956: f64) -> f64 {
    let t73778 = t46917 * t6871;
    let t73781 = t48862 * t48863 * t22298;
    let t73787 = t9962 * t22098;
    let t73789 = t46740 * t22102;
    let t73791 = -0.10289764348336736874e-1_f64 * t5671 * t13789 * t6862 * t13791 + 0.51448821741683684367e-2_f64 * t5671 * t36776 * t13790 * t49146 - 0.85748036236139473944e-2_f64 * t3934 * t13783 * t6816 * t1398 * t3938 + 0.85748036236139473944e-2_f64 * t5671 * t9955 * t22118 * t4004 + 0.34299214494455789578e-2_f64 * t3934 * t13789 * t49146 * t6869 + 0.17149607247227894789e-2_f64 * t3934 * t13789 * t48475 * t6869 + 0.45351183609335988442e-1_f64 * t73778 - 0.11433071498151929859e-3_f64 * t73781 - 0.42874018118069736972e-2_f64 * t3934 * t9955 * t22079 * t9956 - 0.16006300097412701803e-1_f64 * t73787 - 0.10841600599314203355e-2_f64 * t73789;
    t73791
}
