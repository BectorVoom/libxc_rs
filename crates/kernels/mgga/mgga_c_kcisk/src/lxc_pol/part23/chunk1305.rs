//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1305/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1305<F: Float>(t32643: F, t32647: F, t111006: F, t9379: F, t111123: F, t111125: F, t111127: F, t111132: F, t111134: F, t111137: F, t111140: F, t111143: F, t111145: F, t15705: F, t32633: F, t32636: F) -> (F, F) {
    let t111147 = t32647 * t32643;
    let t111149 = t9379 * t111006;
    let t111151 = 0.31250000000000000001e-1 * t111123 + 0.31250000000000000001e-1 * t111125 + 0.14583333333333333334e0 * t111127 + 0.62500000000000000002e-1 * t111132 + 0.14583333333333333334e0 * t111134 + 0.120625e-1 * t111137 - 0.69841875000000000003e-2 * t111140 - 0.69841875000000000003e-2 * t111143 + 0.31250000000000000001e-1 * t111145 + 0.31250000000000000001e-1 * t111147 + 0.10416666666666666667e-1 * t111149;
    let t111153 = t15705 * t32636 * t32633;
    (t111151, t111153)
}
