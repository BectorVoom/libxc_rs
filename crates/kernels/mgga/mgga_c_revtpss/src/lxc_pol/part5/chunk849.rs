//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 849/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk849<F: Float>(t4365: F, t6035: F, t2747: F, t2702: F, t2716: F, t2721: F, t2739: F, t2745: F, t4350: F, t4355: F, t4357: F, t4431: F, t6019: F, t6024: F, t6030: F, t825: F, t851: F) -> (F, F) {
    let t6036 = t4365 * t6035;
    let t6037 = t2747 * t6036;
    let t6040 = -F::cast_from(0.21437009059034868486e-3_f64) * t825 * t6019 + F::cast_from(0.42874018118069736972e-3_f64) * t2721 * t6024 + t2702 + t2716 - F::cast_from(0.10164000561857065645e-3_f64) * t4350 + F::cast_from(0.14291339372689912324e-4_f64) * t4355 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t6030 - t2739 - F::cast_from(0.25410001404642664112e-4_f64) * t4431 + F::cast_from(0.80031500487063509015e-2_f64) * t4357 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t6037;
    (t6037, t6040)
}
