//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1063/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1063<F: Float>(t101: F, t123: F, t1497: F, t1504: F, t1507: F, t1568: F, t1585: F, t1587: F, t1589: F, t1614: F, t16190: F, t1621: F, t1622: F, t16226: F, t16540: F, t16548: F, t16569: F, t16588: F, t16631: F, t16654: F, t16662: F, t16673: F, t16676: F, t16701: F, t16721: F, t204: F, t49: F, t4912: F, t4915: F, t4921: F, t4952: F, t4960: F, t519: F, t527: F, t535: F, t540: F, t541: F, t574: F) -> F {
    let t16732 = -t16548 - t16569 - F::cast_from(0.18989649058080861537e-2_f64) * t49 * t16190 * t123 - F::cast_from(0.35089341735807877242e1_f64) * t1614 * t16588 * t541 + F::cast_from(0.19964560303604640732e6_f64) * t101 / t16673 * t16662 / t16676 + F::cast_from(0.69263436422725855036e2_f64) * t1621 * t4952 * t1507 * t540 + F::cast_from(0.61524113149298439947e4_f64) * t4912 * t1504 * t4915 * t1497 + t16631 - F::cast_from(0.62337092780453269531e3_f64) * t4921 * t1622 * t1497 - F::cast_from(0.24828486201251232145e5_f64) * t101 / t1585 / t1568 * t16662 * t4960 - t16701 + F::cast_from(0.96491876992155210402e2_f64) * t1587 * t16654 * t1589 + F::cast_from(0.6233709278045326953e3_f64) * t4912 * t16540 * t1507 - t16721 + F::cast_from(0.51947577317044391277e2_f64) * t1621 * t16588 * t1507 + F::cast_from(0.5848223622634646207e0_f64) * t535 * t16226 * t541 - F::cast_from(0.21309037037037037036e0_f64) * t204 * t574 * t519 * t527;
    t16732
}
