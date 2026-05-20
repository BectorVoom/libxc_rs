//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1948/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1948<F: Float>(t13999: F, t5677: F, t13967: F, t13977: F, t13981: F, t13987: F, t13988: F, t13991: F, t13995: F, t3934: F, t5671: F, t9847: F, t9896: F, t9906: F, t9910: F, t9919: F) -> F {
    let t14001 = F::cast_from(0.40015750243531754508e-2_f64) * t13999 * t5677;
    let t14002 = F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t13967 + F::cast_from(0.50820002809285328224e-5_f64) * t9847 + F::cast_from(0.10003937560882938627e-2_f64) * t9896 - F::cast_from(0.12705000702321332056e-4_f64) * t9906 - F::cast_from(0.11337795902333997111e-1_f64) * t9910 - F::cast_from(0.20007875121765877254e-2_f64) * t9919 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t13977 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t13981 + t13987 - F::cast_from(0.80031500487063509014e-2_f64) * t13988 + F::cast_from(0.85748036236139473944e-3_f64) * t5671 * t13991 - F::cast_from(0.42874018118069736972e-2_f64) * t3934 * t13995 - t14001;
    t14002
}
