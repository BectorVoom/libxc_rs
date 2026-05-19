//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1061/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1061<F: Float>(t1398: F, t676: F, t14143: F, t14141: F, t10069: F, t5737: F, t5710: F, t72: F, t1432: F, t686: F, t136: F, t1892: F) -> (F, F, F, F) {
    let t14144 = t676 * t1398;
    let t14145 = t14143 * t14144;
    let t14146 = t14141 * t14145;
    let t14149 = t10069 * t5737;
    let t14155 = t5710 * t72;
    let t14158 = F::cast_from(0.19514881078765566038e-1_f64) * t1432 * t14155 * t686;
    let t14159 = t1892 * t136;
    (t14146, t14149, t14158, t14159)
}
