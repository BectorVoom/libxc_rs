//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1063/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1063(t101: f64, t123: f64, t1497: f64, t1504: f64, t1507: f64, t1568: f64, t1585: f64, t1587: f64, t1589: f64, t1614: f64, t16190: f64, t1621: f64, t1622: f64, t16226: f64, t16540: f64, t16548: f64, t16569: f64, t16588: f64, t16631: f64, t16654: f64, t16662: f64, t16673: f64, t16676: f64, t16701: f64, t16721: f64, t204: f64, t49: f64, t4912: f64, t4915: f64, t4921: f64, t4952: f64, t4960: f64, t519: f64, t527: f64, t535: f64, t540: f64, t541: f64, t574: f64) -> f64 {
    let t16732 = -t16548 - t16569 - 0.18989649058080861537e-2_f64 * t49 * t16190 * t123 - 0.35089341735807877242e1_f64 * t1614 * t16588 * t541 + 0.19964560303604640732e6_f64 * t101 / t16673 * t16662 / t16676 + 0.69263436422725855036e2_f64 * t1621 * t4952 * t1507 * t540 + 0.61524113149298439947e4_f64 * t4912 * t1504 * t4915 * t1497 + t16631 - 0.62337092780453269531e3_f64 * t4921 * t1622 * t1497 - 0.24828486201251232145e5_f64 * t101 / t1585 / t1568 * t16662 * t4960 - t16701 + 0.96491876992155210402e2_f64 * t1587 * t16654 * t1589 + 0.6233709278045326953e3_f64 * t4912 * t16540 * t1507 - t16721 + 0.51947577317044391277e2_f64 * t1621 * t16588 * t1507 + 0.5848223622634646207e0_f64 * t535 * t16226 * t541 - 0.21309037037037037036e0_f64 * t204 * t574 * t519 * t527;
    t16732
}
