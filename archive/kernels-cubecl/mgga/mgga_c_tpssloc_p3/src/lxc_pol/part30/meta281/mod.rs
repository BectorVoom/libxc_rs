//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1272;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1273;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta281<F: Float>(t1998: F, t7708: F, t6926: F, t1339: F, t1825: F, t6936: F, t1814: F, t2002: F, t559: F, t1827: F, t6945: F, t1831: F, t6952: F, t6915: F, t6922: F, t6935: F, t6949: F, t7706: F, t539: F, t1842: F, t2015: F, t3887: F) -> (F, F, F, F, F, F) {
        let (t7709, t7710, t7712, t7713, t7715, t7716, t7718, t7720) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1272::<F>(t1998, t7708, t6926, t1339, t1825, t6936, t1814, t2002, t559, t1827, t6945, t1831, t6952);
        let t7722 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1273::<F>(t6915, t6922, t6935, t6949, t7706, t7710, t7713, t7716, t7718, t7720);
        let (t7723, t7729) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1274::<F>(t539, t7722, t1842, t2015, t3887);
    (t7709, t7712, t7715, t7722, t7723, t7729)
}
