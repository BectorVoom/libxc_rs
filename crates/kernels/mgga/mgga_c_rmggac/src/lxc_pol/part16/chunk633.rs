//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 633/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk633<F: Float>(t515: F, t9843: F, t1971: F, t7230: F, t2310: F, t8571: F, t2320: F, t9222: F, t2295: F, t6355: F, t1704: F, t27: F, t649: F, t7282: F, t2301: F, t2868: F) -> (F, F, F, F, F, F, F, F) {
    let t9844 = t515 * t9843;
    let t9845 = t1971 * t9844;
    let t9846 = t7230 * t9845;
    let t9848 = t8571 * t2310;
    let t9850 = t9222 * t2320;
    let t9861 = t6355 * t2295;
    let t9864 = t27 * t649 * t1704;
    let t9865 = t7282 * t9864;
    let t9870 = t2868 * t2301;
    (t9845, t9846, t9848, t9850, t9861, t9864, t9865, t9870)
}
