//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1387/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1387<F: Float>(t2799: F, t60200: F, t18175: F, t9696: F, t1957: F, t34374: F, t5218: F, t11691: F, t9988: F, t17775: F, t33086: F, t33130: F, t7293: F, t10039: F, t112139: F, t112167: F, t112173: F, t117597: F, t117601: F, t117627: F, t117650: F, t117686: F, t117732: F, t117759: F, t117781: F, t117815: F, t117847: F, t117889: F, t117911: F, t117946: F, t117980: F, t118009: F, t118044: F, t118068: F, t118092: F, t118119: F, t118148: F, t118168: F, t118200: F, t118229: F, t118256: F, t118282: F, t118309: F, t118340: F, t118368: F, t118394: F, t118428: F, t118449: F, t118487: F, t118506: F, t118532: F, t12352: F, t18182: F, t18839: F, t2049: F, t2666: F, t33151: F, t33153: F, t34377: F, t34612: F, t34650: F, t5527: F, t5533: F, t5552: F, t7656: F, t7659: F, t802: F) -> (F, F, F, F, F, F, F) {
    let t118541 = t60200 * t2799;
    let t118543 = t9696 * t18175;
    let t118548 = 4.0 * t5218 * t34374 * t1957;
    let t118549 = t11691 * t9988;
    let t118556 = 2.0 * t17775 * t33086;
    let t118558 = t7293 * t33130;
    let t118559 = -6.0 * t12352 * t10039 * t5533 + 4.0 * t112139 * t7659 + t117597 - t112167 * t2666 + t117601 + (t118068 + t117781 + t118229 + t118282 + t117889 + t117732 + t118200 + t118449 + t117847 + t118368 + t117627 + t118148 + t118044 + t117980 + t118256 + t118009 + t118428 + t117759 + t117946 + t118506 + t118309 + t118532 + t118487 + t117911 + t118340 + t118092 + t117650 + t118119 + t117815 + t118394 + t118168 + t117686) * t802 - 2.0 * t5527 * t34650 + t118541 - t7656 * t33151 + t118543 - 6.0 * t112173 * t18182 - t118548 + t118549 - 12.0 * t12352 * t34612 * t2049 + 4.0 * t33153 * t18839 - t118556 - t34377 * t5552 + t118558;
    (t118541, t118543, t118548, t118549, t118556, t118558, t118559)
}
