//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2118/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2118<F: Float>(t18643: F, t92955: F, t18456: F, t27261: F, t6037: F, t92951: F, t18521: F, t25222: F, t6030: F, t103264: F, t92963: F, t92966: F, t92969: F, t92976: F, t98968: F, t98973: F) -> F {
    let t106006 = t92955 * t18643;
    let t106008 = t27261 * t18456;
    let t106010 = t92951 * t6037;
    let t106012 = t27261 * t18521;
    let t106014 = t25222 * t6030;
    let t106020 = F::cast_from(0.2032800112371413129e-3_f64) * t106006 + F::cast_from(0.25724410870841842183e-2_f64) * t106008 - F::cast_from(0.16006300097412701803e-1_f64) * t106010 + F::cast_from(0.85748036236139473944e-3_f64) * t106012 + F::cast_from(0.80031500487063509015e-2_f64) * t106014 + F::cast_from(0.50820002809285328225e-5_f64) * t92963 - F::cast_from(0.36143185997963725434e-4_f64) * t92966 - F::new(35.0) / F::new(216.0) * t92969 - t103264 - F::cast_from(0.57165357490759649295e-3_f64) * t98968 - t98973 + t92976;
    t106020
}
