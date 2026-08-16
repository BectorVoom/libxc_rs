//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2670/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2670(t1398: f64, t5658: f64, t13783: f64, t13789: f64, t13790: f64, t1872: f64, t3829: f64, t3934: f64, t3938: f64, t46730: f64, t47318: f64, t47320: f64, t47325: f64, t47329: f64, t47333: f64, t47337: f64, t47338: f64, t49118: f64, t49122: f64, t49125: f64, t49127: f64, t49128: f64, t49134: f64, t49139: f64, t49144: f64, t5671: f64, t800: f64, t9400: f64) -> (f64, f64) {
    let t49146 = t5658 * t1398;
    let t49157 = -0.42874018118069736972e-4_f64 * t47318 + 0.18292914397043087775e-2_f64 * t47320 + 0.42874018118069736972e-3_f64 * t47325 - 0.85748036236139473944e-4_f64 * t47329 - 0.12705000702321332056e-4_f64 * t47333 + 0.60023625365297631762e-2_f64 * t49118 - t49122 - t49125 - t49127 + 7.0_f64 / 4.0_f64 * t49128 + 5.0_f64 / 4.0_f64 * t46730 * t800 * t1872 * t9400 + 7.0_f64 / 48.0_f64 * t49134 + 0.34299214494455789577e-3_f64 * t49139 + 0.21437009059034868486e-4_f64 * t49144 + 0.51448821741683684367e-2_f64 * t3934 * t13789 * t49146 * t3938 + 0.25724410870841842183e-1_f64 * t5671 * t13783 * t13790 * t3829 * t1398 + t47337 - 35.0_f64 / 72.0_f64 * t47338;
    (t49146, t49157)
}
