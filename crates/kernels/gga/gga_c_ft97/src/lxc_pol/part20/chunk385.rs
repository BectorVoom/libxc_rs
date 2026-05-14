//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 385/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk385<F: Float>(t3977: F, t766: F, t242: F, t1168: F, t2469: F, t1170: F, t1882: F, t1144: F, t1175: F, t713: F, t729: F, t1131: F, t773: F, t265: F, t3821: F, t1901: F, t193: F, t3877: F, t3882: F, t3888: F, t3894: F, t3899: F, t3953: F, t3958: F, t3974: F, t446: F, t89: F) -> (F, F, F, F, F, F) {
    let t3978 = t3977 * t766;
    let t3979 = t242 * t3978;
    let t3982 = t2469 * t1168;
    let t3983 = t242 * t3982;
    let t3986 = t1882 * t1170;
    let t3988 = t1882 * t1144;
    let t3991 = t729 * t1175 * t713;
    let t3995 = t729 * t773 * t1131;
    let t3999 = t729 * t265 * t3821;
    let t4002 = t1901 * t3877 / 9.0 + t1901 * t3882 / 9.0 + 2.0 / 9.0 * t1901 * t3888 - 2.0 / 27.0 * t1901 * t3894 + t1901 * t3899 / 9.0 + t89 * t193 * t3953 / 3.0 - t3958 / 9.0 - t446 * t3974 / 3.0 - t446 * t3979 / 3.0 - t446 * t3983 / 3.0 + t3986 / 9.0 + t3988 / 9.0 - t446 * t3991 / 3.0 - t446 * t3995 / 3.0 - t446 * t3999 / 3.0;
    (t3979, t3983, t3991, t3995, t3999, t4002)
}
