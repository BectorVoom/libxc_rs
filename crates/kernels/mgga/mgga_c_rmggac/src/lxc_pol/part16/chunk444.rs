//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 444/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk444<F: Float>(t219: F, t4467: F, t4462: F, t612: F, t1228: F, t1522: F, t4555: F, t608: F, t1477: F, t4559: F, t1193: F, t5582: F, t1503: F, t31: F, t4518: F, t1466: F) -> (F, F, F, F, F, F, F) {
    let t5672 = t4467 * t219;
    let t5677 = t4462 * t612;
    let t5681 = 0.25610252642437845428e0 * t1228 * t1522;
    let t5685 = t4555 * t608;
    let t5693 = 0.25610252642437845428e0 * t4559 * t1477;
    let t5694 = t1193 * t5582;
    let t5696 = 0.12805126321218922714e0 * t5694 * t1503;
    let t5697 = t4518 * t31;
    let t5698 = t5697 * t1466;
    (t5672, t5677, t5681, t5685, t5693, t5696, t5698)
}
