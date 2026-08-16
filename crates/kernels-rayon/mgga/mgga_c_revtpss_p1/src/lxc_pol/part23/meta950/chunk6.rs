//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3148/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3148(t17401: f64, t20926: f64, t4890: f64, t70993: f64, t12787: f64, t17709: f64, t17729: f64, t20956: f64, t24840: f64, t3362: f64, t3720: f64, t3767: f64, t3782: f64, t4181: f64, t44664: f64, t5335: f64, t5343: f64, t5354: f64, t6587: f64, t69787: f64, t69789: f64, t69793: f64, t69812: f64, t71081: f64, t72086: f64) -> f64 {
    let t82678 = t17401 * t20926;
    let t82680 = t70993 * t4890;
    let t82696 = 0.38586616306262763276e-2_f64 * t17709 * t3720 * t20956 * t72086 - 0.57165357490759649295e-3_f64 * t69787 + 0.30488190661738479624e-2_f64 * t69789 + 0.64311027177104605458e-3_f64 * t44664 * t24840 - 0.85748036236139473947e-3_f64 * t82678 + 0.43445671692977333464e-1_f64 * t3767 * t82680 * t5343 - 0.21722835846488666732e-1_f64 * t3782 * t82680 * t5335 + 0.68598428988911579154e-2_f64 * t71081 * t5354 - 0.7145669686344956162e-3_f64 * t17729 * t12787 * t6587 * t3362 * t4181 + 0.42874018118069736972e-3_f64 * t69793 - 0.30488190661738479624e-2_f64 * t69812;
    t82696
}
