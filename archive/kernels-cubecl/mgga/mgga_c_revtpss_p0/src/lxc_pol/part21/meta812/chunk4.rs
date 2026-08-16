//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2973/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2973<F: Float>(t15728: F, t15827: F, t11672: F, t15984: F, t1042: F, t11231: F, t11637: F, t11703: F, t11994: F, t16089: F, t16095: F, t16138: F, t16167: F, t20094: F, t20099: F, t2862: F, t3092: F, t3127: F, t42417: F, t42695: F, t42754: F, t42756: F, t4783: F, t4912: F) -> F {
    let t54198 = t15728 * t15827;
    let t54222 = t11672 * t15984;
    let t54224 = -F::cast_from(77.0_f64) / F::cast_from(486.0_f64) * t42754 - F::cast_from(11.0_f64) / F::cast_from(162.0_f64) * t42756 - F::cast_from(0.91464571985215438873e-2_f64) * t54198 + F::cast_from(0.85748036236139473944e-3_f64) * t16095 * t3092 * t20094 * t11231 - F::cast_from(0.71456696863449561621e-3_f64) * t16095 * t11703 * t20099 * t11231 + F::cast_from(0.14291339372689912324e-2_f64) * t16089 * t11703 * t20099 * t11637 - F::cast_from(0.42874018118069736972e-3_f64) * t11994 * t16167 - F::cast_from(0.42874018118069736972e-3_f64) * t3127 * t1042 * t16138 * t2862 - F::cast_from(0.21722835846488666732e-1_f64) * t42695 * t4912 + F::cast_from(0.14481890564325777821e-1_f64) * t42417 * t4783 - F::cast_from(0.30488190661738479624e-2_f64) * t54222;
    t54224
}
