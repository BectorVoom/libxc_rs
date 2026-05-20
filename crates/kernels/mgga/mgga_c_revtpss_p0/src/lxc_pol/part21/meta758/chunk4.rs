//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2670/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2670<F: Float>(t1398: F, t5658: F, t13783: F, t13789: F, t13790: F, t1872: F, t3829: F, t3934: F, t3938: F, t46730: F, t47318: F, t47320: F, t47325: F, t47329: F, t47333: F, t47337: F, t47338: F, t49118: F, t49122: F, t49125: F, t49127: F, t49128: F, t49134: F, t49139: F, t49144: F, t5671: F, t800: F, t9400: F) -> (F, F) {
    let t49146 = t5658 * t1398;
    let t49157 = -F::cast_from(0.42874018118069736972e-4_f64) * t47318 + F::cast_from(0.18292914397043087775e-2_f64) * t47320 + F::cast_from(0.42874018118069736972e-3_f64) * t47325 - F::cast_from(0.85748036236139473944e-4_f64) * t47329 - F::cast_from(0.12705000702321332056e-4_f64) * t47333 + F::cast_from(0.60023625365297631762e-2_f64) * t49118 - t49122 - t49125 - t49127 + F::new(7.0) / F::new(4.0) * t49128 + F::new(5.0) / F::new(4.0) * t46730 * t800 * t1872 * t9400 + F::new(7.0) / F::new(48.0) * t49134 + F::cast_from(0.34299214494455789577e-3_f64) * t49139 + F::cast_from(0.21437009059034868486e-4_f64) * t49144 + F::cast_from(0.51448821741683684367e-2_f64) * t3934 * t13789 * t49146 * t3938 + F::cast_from(0.25724410870841842183e-1_f64) * t5671 * t13783 * t13790 * t3829 * t1398 + t47337 - F::new(35.0) / F::new(72.0) * t47338;
    (t49146, t49157)
}
