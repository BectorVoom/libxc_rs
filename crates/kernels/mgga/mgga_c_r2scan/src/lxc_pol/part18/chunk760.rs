//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 760/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk760<F: Float>(t2287: F, t757: F, t6007: F, t2289: F, t2292: F, t2300: F, t357: F, t862: F, t2304: F, t268: F, t270: F, t1266: F, t359: F) -> (F, F, F, F, F) {
    let t6835 = F::cast_from(1.0_f64) / t2287 / t757;
    let t6836 = t6835 * t6007;
    let t6839 = t2289 * t2292;
    let t6842 = t357 * t2300;
    let t6843 = t862 * t6842;
    let t6845 = t268 * t270 * t2304;
    let t6848 = t359 * t1266;
    (t6836, t6839, t6843, t6845, t6848)
}
