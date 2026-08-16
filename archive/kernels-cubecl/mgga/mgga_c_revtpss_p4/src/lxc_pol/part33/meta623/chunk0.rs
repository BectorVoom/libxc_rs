//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2062/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2062<F: Float>(t25331: F, t27216: F, t212: F, t27265: F, t689: F, t780: F, t1568: F, t7063: F, t25410: F, t25413: F, t27299: F, t93281: F) -> (F, F, F, F, F, F, F) {
    let t98825 = t27216 * t25331;
    let t98830 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t212 * t27265 * t780;
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98851 = F::cast_from(0.25702851531048074406e-1_f64) * t98849 * t25413;
    let t98852 = t27299 * t689;
    let t98853 = t93281 * t98852;
    (t98825, t98830, t98848, t98849, t98851, t98852, t98853)
}
