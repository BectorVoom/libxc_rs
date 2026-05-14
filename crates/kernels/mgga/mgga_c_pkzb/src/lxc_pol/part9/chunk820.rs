//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 820/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk820<F: Float>(t2185: F, t824: F, t218: F, t219: F, t334: F, t6106: F, t6090: F, t6093: F, t6108: F, t6151: F, t6154: F, t6159: F, t6161: F, t6166: F, t6169: F, t6171: F, t6175: F, t6177: F, t6180: F, t6183: F) -> (F, F, F, F, F) {
    let t6185 = t824 * t2185;
    let t6187 = t218 * t219 * t6185;
    let t6189 = t334 * t6106;
    let t6191 = t218 * t219 * t6189;
    let t6193 = 0.142419375e1 * t6151 - 0.28483875e1 * t6154 + 0.1898925e1 * t6159 - t6161 + 0.11958666666666666667e1 * t6090 - 0.89690000000000000001e0 * t6093 + 0.8969e0 * t6108 - 0.76790625e-1 * t6166 + 0.46074375e0 * t6169 + 0.3071625e0 * t6171 - t6175 + 0.82156666666666666666e0 * t6177 - 0.49293999999999999999e0 * t6180 - 0.49293999999999999999e0 * t6183 + 0.73941e0 * t6187 + 0.24647e0 * t6191;
    (t6185, t6187, t6189, t6191, t6193)
}
