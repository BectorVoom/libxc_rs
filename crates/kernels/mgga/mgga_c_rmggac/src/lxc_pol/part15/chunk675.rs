//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 675/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk675<F: Float>(t34704: F, t7335: F, t7766: F, t7334: F, t7552: F, t7558: F, t7352: F, t934: F, t7197: F, t892: F, t7203: F, t899: F, t20: F, t4764: F, t132: F, t1327: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34705 = 0.91462949374725084942e-3 * t34704;
    let t34706 = t7335 * t7766;
    let t34707 = 0.45731474687362542471e-3 * t34706;
    let t34709 = t7334 * t7552;
    let t34710 = t34709 * t7558;
    let t34711 = 0.65053455985619242968e-4 * t34710;
    let t34715 = t934 * t7352;
    let t34724 = t892 * t7197;
    let t34735 = t892 * t7203;
    let t34738 = t899 * t7203;
    let t34747 = t20 * t4764;
    let t34750 = t132 * t1327;
    (t34705, t34707, t34711, t34715, t34724, t34735, t34738, t34747, t34750)
}
