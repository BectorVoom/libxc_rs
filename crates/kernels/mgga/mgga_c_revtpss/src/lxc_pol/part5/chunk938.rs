//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 938/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk938<F: Float>(t760: F, t9387: F, t2496: F, t2523: F, t9372: F, t37: F, t716: F, t2626: F, t9425: F, t2609: F, t606: F, t706: F, t775: F, t853: F, t2710: F, t2793: F, t9285: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10596 = 0.5848223622634646207e0 * t760 * t9387;
    let t10597 = t2523 * t2496;
    let t10604 = 0.10254018858216406658e4 * t760 * t9372;
    let t10605 = t37 * t716;
    let t10608 = t2523 * t2626;
    let t10611 = 0.35089341735807877242e1 * t760 * t9425;
    let t10612 = t2609 * t606;
    let t10613 = t706 * t10612;
    let t10631 = t853 * t775;
    let t10645 = 0.46263278077393568556e-2 * t2710 * t2793 * t9285;
    (t10596, t10597, t10604, t10605, t10608, t10611, t10613, t10631, t10645)
}
