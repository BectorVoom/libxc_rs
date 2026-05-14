//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 571/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk571<F: Float>(t4365: F, t6035: F, t2747: F, t2702: F, t2716: F, t2721: F, t2739: F, t2745: F, t4350: F, t4355: F, t4357: F, t4431: F, t6019: F, t6024: F, t6030: F, t825: F, t851: F) -> (F, F) {
    let t6036 = t4365 * t6035;
    let t6037 = t2747 * t6036;
    let t6040 = -0.21437009059034868486e-3 * t825 * t6019 + 0.42874018118069736972e-3 * t2721 * t6024 + t2702 + t2716 - 0.10164000561857065645e-3 * t4350 + 0.14291339372689912324e-4 * t4355 - 0.85748036236139473944e-3 * t851 * t6030 - t2739 - 0.25410001404642664112e-4 * t4431 + 0.80031500487063509015e-2 * t4357 + 0.17149607247227894789e-2 * t2745 * t6037;
    (t6037, t6040)
}
