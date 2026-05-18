//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 949/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk949<F: Float>(t1797: F, t5185: F, t19112: F, t359: F, t376: F, t1170: F, t18458: F, t381: F, t1189: F, t1175: F, t6696: F, t1796: F, t284: F) -> (F, F, F, F, F) {
    let t20139 = t1797 * t5185;
    let t20141 = t359 * t19112;
    let t20142 = t376 * t20141;
    let t20143 = t1170 * t20142;
    let t20145 = t18458 * t381;
    let t20146 = t20145 * t1189;
    let t20148 = t1175 * t6696;
    let t20149 = t1170 * t20148;
    let t20151 = t1796 * t284;
    (t20139, t20143, t20146, t20149, t20151)
}
