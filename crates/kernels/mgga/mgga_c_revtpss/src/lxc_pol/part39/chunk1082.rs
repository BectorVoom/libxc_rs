//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1082/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1082<F: Float>(t5674: F, t9955: F, t9956: F, t4000: F, t820: F, t844: F, t5677: F, t13967: F, t13977: F, t13981: F, t13987: F, t13988: F, t13991: F, t3934: F, t5671: F, t9847: F, t9896: F, t9906: F, t9910: F, t9919: F) -> (F,) {
    let t13995 = t9955 * t5674 * t9956;
    let t13999 = t820 * t4000 * t844;
    let t14001 = 0.40015750243531754508e-2 * t13999 * t5677;
    let t14002 = 0.85748036236139473944e-3 * t3934 * t13967 + 0.50820002809285328224e-5 * t9847 + 0.10003937560882938627e-2 * t9896 - 0.12705000702321332056e-4 * t9906 - 0.11337795902333997111e-1 * t9910 - 0.20007875121765877254e-2 * t9919 + 0.17149607247227894789e-2 * t3934 * t13977 + 0.85748036236139473944e-3 * t3934 * t13981 + t13987 - 0.80031500487063509014e-2 * t13988 + 0.85748036236139473944e-3 * t5671 * t13991 - 0.42874018118069736972e-2 * t3934 * t13995 - t14001;
    (t14002,)
}
