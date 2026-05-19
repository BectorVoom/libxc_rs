//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 858/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk858<F: Float>(t2241: F, t351: F, t6143: F, t6199: F, t2189: F, t832: F, t853: F, t6087: F, t6174: F, t6090: F, t6093: F, t6108: F, t6151: F, t6154: F, t6159: F, t6166: F, t6169: F, t6171: F, t6177: F, t6180: F, t6183: F, t6187: F, t6191: F) -> (F, F, F, F, F, F, F, F) {
    let t6201 = F::new(1.0) / t2241 / t351;
    let t6202 = t6143 * t6201;
    let t6204 = F::cast_from(0.51726012919273400301e3_f64) * t6199 * t6202;
    let t6205 = t2189 * t832;
    let t6207 = F::new(3.0) * t6205 * t853;
    let t6211 = F::cast_from(0.93932222222222222223e0_f64) * t6087;
    let t6218 = F::cast_from(0.36793333333333333333e0_f64) * t6174;
    let t6224 = F::new(0.19419375e1) * t6151 - F::new(0.3883875e1) * t6154 + F::new(0.258925e1) * t6159 - t6211 + F::new(0.12077e1) * t6090 - F::new(0.905775e0) * t6093 + F::new(0.905775e0) * t6108 - F::cast_from(0.412621875e-1_f64) * t6166 + F::cast_from(0.247573125e0_f64) * t6169 + F::new(0.16504875e0) * t6171 - t6218 + F::new(0.82785e0) * t6177 - F::new(0.49671e0) * t6180 - F::new(0.49671e0) * t6183 + F::new(0.745065e0) * t6187 + F::new(0.248355e0) * t6191;
    (t6201, t6202, t6204, t6205, t6207, t6211, t6218, t6224)
}
