//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1111;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1112;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1113;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta256<F: Float>(t25: F, t265: F, t394: F, t202: F, t7109: F, t1877: F, t193: F, t2057: F, t2522: F, t7114: F, t776: F, t868: F, t870: F, t2064: F, t40: F, t606: F, t607: F, t6542: F, t6671: F, t7110: F, dens_threshold: F, rho0: F, zeta_threshold: F, t28: F, t504: F, t1081: F, t2071: F, t52: F, t6841: F, t6848: F, rho1: F, t1268: F, t2039: F, t2314: F, t5113: F, t671: F, t7040: F, t7042: F, t7056: F, t2094: F, t532: F) -> (F, F, F, F, F, F) {
        let (t7125, t7130, t7131, t7136) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1111::<F>(t25, t265, t394, t202, t7109, t1877, t193, t2057, t2522, t7114, t776, t868, t870, t2064, t40, t606, t607, t6542, t6671, t7110, dens_threshold, rho0, zeta_threshold);
        let (t7150, t7155) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1112::<F>(t28, t265, t504, t7130, t1081, t1877, t2057, t2071, t2522, t52, t607, t6841, t6848, t7110, t7114, dens_threshold, rho1, zeta_threshold);
        let t7156 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1113::<F>(t7136, t7155);
        let (t7166, t7170) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1114::<F>(t1268, t2039, t2314, t5113, t671, t7040, t7042, t7056, t2094, t532);
    (t7125, t7131, t7150, t7156, t7166, t7170)
}
