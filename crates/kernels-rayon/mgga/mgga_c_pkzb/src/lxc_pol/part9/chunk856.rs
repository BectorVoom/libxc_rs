//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 856/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk856(t2185: f64, t824: f64, t218: f64, t219: f64, t334: f64, t6106: f64, t6090: f64, t6093: f64, t6108: f64, t6151: f64, t6154: f64, t6159: f64, t6161: f64, t6166: f64, t6169: f64, t6171: f64, t6175: f64, t6177: f64, t6180: f64, t6183: f64) -> (f64, f64, f64, f64, f64) {
    let t6185 = t824 * t2185;
    let t6187 = t218 * t219 * t6185;
    let t6189 = t334 * t6106;
    let t6191 = t218 * t219 * t6189;
    let t6193 = 0.142419375e1_f64 * t6151 - 0.28483875e1_f64 * t6154 + 0.1898925e1_f64 * t6159 - t6161 + 0.11958666666666666667e1_f64 * t6090 - 0.89690000000000000001e0_f64 * t6093 + 0.8969e0_f64 * t6108 - 0.76790625e-1_f64 * t6166 + 0.46074375e0_f64 * t6169 + 0.3071625e0_f64 * t6171 - t6175 + 0.82156666666666666666e0_f64 * t6177 - 0.49293999999999999999e0_f64 * t6180 - 0.49293999999999999999e0_f64 * t6183 + 0.73941e0_f64 * t6187 + 0.24647e0_f64 * t6191;
    (t6185, t6187, t6189, t6191, t6193)
}
