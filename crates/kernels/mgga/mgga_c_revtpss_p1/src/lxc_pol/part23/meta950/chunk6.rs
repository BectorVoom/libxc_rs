//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3148/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3148<F: Float>(t17401: F, t20926: F, t4890: F, t70993: F, t12787: F, t17709: F, t17729: F, t20956: F, t24840: F, t3362: F, t3720: F, t3767: F, t3782: F, t4181: F, t44664: F, t5335: F, t5343: F, t5354: F, t6587: F, t69787: F, t69789: F, t69793: F, t69812: F, t71081: F, t72086: F) -> F {
    let t82678 = t17401 * t20926;
    let t82680 = t70993 * t4890;
    let t82696 = F::cast_from(0.38586616306262763276e-2_f64) * t17709 * t3720 * t20956 * t72086 - F::cast_from(0.57165357490759649295e-3_f64) * t69787 + F::cast_from(0.30488190661738479624e-2_f64) * t69789 + F::cast_from(0.64311027177104605458e-3_f64) * t44664 * t24840 - F::cast_from(0.85748036236139473947e-3_f64) * t82678 + F::cast_from(0.43445671692977333464e-1_f64) * t3767 * t82680 * t5343 - F::cast_from(0.21722835846488666732e-1_f64) * t3782 * t82680 * t5335 + F::cast_from(0.68598428988911579154e-2_f64) * t71081 * t5354 - F::cast_from(0.7145669686344956162e-3_f64) * t17729 * t12787 * t6587 * t3362 * t4181 + F::cast_from(0.42874018118069736972e-3_f64) * t69793 - F::cast_from(0.30488190661738479624e-2_f64) * t69812;
    t82696
}
