//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2153/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2153<F: Float>(t19976: F, t25580: F, t19900: F, t7111: F, t100030: F, t19718: F, t19831: F, t19973: F, t19982: F, t20070: F, t20075: F, t20091: F, t27493: F, t27498: F, t93658: F, t93667: F, t93745: F, t93750: F) -> F {
    let t107086 = t25580 * t19976;
    let t107101 = t7111 * t19900;
    let t107103 = F::cast_from(0.95275595817932748827e-3_f64) * t100030 * t19982 - F::cast_from(0.57165357490759649296e-3_f64) * t107086 - F::cast_from(0.85748036236139473944e-3_f64) * t27498 * t19718 - F::cast_from(0.85748036236139473944e-3_f64) * t25580 * t20091 + t93745 / F::new(162.0) + t93750 + F::cast_from(0.85748036236139473944e-3_f64) * t93667 * t19831 + F::cast_from(0.17149607247227894789e-2_f64) * t27493 * t19973 - F::cast_from(0.42874018118069736972e-3_f64) * t27498 * t20070 - F::cast_from(0.85748036236139473944e-3_f64) * t93658 * t20075 - t107101 / F::new(432.0);
    t107103
}
