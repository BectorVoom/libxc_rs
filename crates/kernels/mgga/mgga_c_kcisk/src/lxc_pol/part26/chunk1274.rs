//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1274/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1274<F: Float>(t3936: F, t9427: F, t6174: F, t9452: F, t20233: F, t33359: F, t32087: F, t32008: F, t13485: F, t33423: F, t12951: F, t1328: F, t33597: F, t3739: F, t33610: F, t33509: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113721 = t3936 * t9427;
    let t113727 = t6174 * t9427;
    let t113735 = t3936 * t9452;
    let t113745 = t20233 * t33359;
    let t113747 = 0.23148148148148148148e-2 * t32087 * t113745;
    let t113800 = t32008 * t113745;
    let t113805 = t32087 * t13485 * t33423;
    let t113832 = t1328 * t12951;
    let t113853 = t3739 * t33597;
    let t113854 = 0.22109259259259259258e-2 * t113853;
    let t113855 = t3739 * t33610;
    let t113856 = 0.66327777777777777776e-2 * t113855;
    let t113857 = t3739 * t33509;
    (t113721, t113727, t113735, t113747, t113800, t113805, t113832, t113853, t113854, t113855, t113856, t113857)
}
