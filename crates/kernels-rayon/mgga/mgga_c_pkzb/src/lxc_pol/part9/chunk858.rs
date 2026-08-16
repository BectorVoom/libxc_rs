//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 858/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk858(t2241: f64, t351: f64, t6143: f64, t6199: f64, t2189: f64, t832: f64, t853: f64, t6087: f64, t6174: f64, t6090: f64, t6093: f64, t6108: f64, t6151: f64, t6154: f64, t6159: f64, t6166: f64, t6169: f64, t6171: f64, t6177: f64, t6180: f64, t6183: f64, t6187: f64, t6191: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6201 = 1.0_f64 / t2241 / t351;
    let t6202 = t6143 * t6201;
    let t6204 = 0.51726012919273400301e3_f64 * t6199 * t6202;
    let t6205 = t2189 * t832;
    let t6207 = 3.0_f64 * t6205 * t853;
    let t6211 = 0.93932222222222222223e0_f64 * t6087;
    let t6218 = 0.36793333333333333333e0_f64 * t6174;
    let t6224 = 0.19419375e1_f64 * t6151 - 0.3883875e1_f64 * t6154 + 0.258925e1_f64 * t6159 - t6211 + 0.12077e1_f64 * t6090 - 0.905775e0_f64 * t6093 + 0.905775e0_f64 * t6108 - 0.412621875e-1_f64 * t6166 + 0.247573125e0_f64 * t6169 + 0.16504875e0_f64 * t6171 - t6218 + 0.82785e0_f64 * t6177 - 0.49671e0_f64 * t6180 - 0.49671e0_f64 * t6183 + 0.745065e0_f64 * t6187 + 0.248355e0_f64 * t6191;
    (t6201, t6202, t6204, t6205, t6207, t6211, t6218, t6224)
}
