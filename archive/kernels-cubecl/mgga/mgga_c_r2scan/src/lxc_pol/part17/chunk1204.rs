//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1204/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1204<F: Float>(t12735: F, t12738: F, t12741: F, t12744: F, t12745: F, t41124: F, t41126: F, t41127: F, t41128: F, t41129: F, t41130: F, t41131: F, t41132: F, t41133: F, t41134: F, t41135: F) -> F {
    let t44009 = t41124 - t41126 + t41127 + t41128 + t41129 + t41130 + t12735 - t12738 + t12741 + t12744 + t12745 + t41131 + t41132 + t41133 - t41134 + t41135;
    t44009
}
