//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3838/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3838<F: Float>(t46917: F, t6871: F, t22298: F, t48862: F, t48863: F, t22098: F, t9962: F, t22102: F, t46740: F, t13783: F, t13789: F, t13790: F, t13791: F, t1398: F, t22079: F, t22118: F, t36776: F, t3934: F, t3938: F, t4004: F, t48475: F, t49146: F, t5671: F, t6816: F, t6862: F, t6869: F, t9955: F, t9956: F) -> F {
    let t73778 = t46917 * t6871;
    let t73781 = t48862 * t48863 * t22298;
    let t73787 = t9962 * t22098;
    let t73789 = t46740 * t22102;
    let t73791 = -F::cast_from(0.10289764348336736874e-1_f64) * t5671 * t13789 * t6862 * t13791 + F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t36776 * t13790 * t49146 - F::cast_from(0.85748036236139473944e-2_f64) * t3934 * t13783 * t6816 * t1398 * t3938 + F::cast_from(0.85748036236139473944e-2_f64) * t5671 * t9955 * t22118 * t4004 + F::cast_from(0.34299214494455789578e-2_f64) * t3934 * t13789 * t49146 * t6869 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t13789 * t48475 * t6869 + F::cast_from(0.45351183609335988442e-1_f64) * t73778 - F::cast_from(0.11433071498151929859e-3_f64) * t73781 - F::cast_from(0.42874018118069736972e-2_f64) * t3934 * t9955 * t22079 * t9956 - F::cast_from(0.16006300097412701803e-1_f64) * t73787 - F::cast_from(0.10841600599314203355e-2_f64) * t73789;
    t73791
}
