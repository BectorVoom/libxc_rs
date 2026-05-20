//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2117/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2117<F: Float>(t18446: F, t25270: F, t18629: F, t18428: F, t27261: F, t18651: F, t18639: F, t105985: F, t105987: F, t105989: F, t105991: F, t105993: F, t98937: F, t98950: F) -> F {
    let t105995 = t25270 * t18446;
    let t105997 = t25270 * t18629;
    let t105999 = t27261 * t18428;
    let t106001 = t25270 * t18651;
    let t106003 = t25270 * t18639;
    let t106005 = -F::cast_from(0.80031500487063509016e-2_f64) * t98937 + F::cast_from(0.17149607247227894789e-2_f64) * t105985 - t98950 - F::cast_from(0.85748036236139473944e-3_f64) * t105987 + F::cast_from(0.34299214494455789578e-2_f64) * t105989 - F::cast_from(0.25724410870841842183e-2_f64) * t105991 - F::cast_from(0.85748036236139473945e-2_f64) * t105993 + F::cast_from(0.17149607247227894789e-2_f64) * t105995 + F::cast_from(0.17149607247227894789e-2_f64) * t105997 - F::cast_from(0.34299214494455789578e-2_f64) * t105999 - F::cast_from(0.42874018118069736972e-3_f64) * t106001 + F::cast_from(0.34299214494455789578e-2_f64) * t106003;
    t106005
}
