//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1224;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1225;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1226;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1227;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta252<F: Float>(t265: F, t394: F, t202: F, t6665: F, t1877: F, t1915: F, t193: F, t2522: F, t6670: F, t776: F, t868: F, t870: F, t1068: F, t1070: F, t336: F, t4700: F, t6818: F, t6822: F, t25: F, t1965: F, t40: F, t607: F, t6678: F, t28: F, t1081: F, t6666: F, dens_threshold: F, rho0: F, zeta_threshold: F, t504: F, t1972: F, t52: F, rho1: F, t1873: F, t2314: F, t5113: F, t1268: F, t6534: F, t6515: F, t6517: F, t671: F, t1271: F, t191: F, t192: F, t2020: F, t2018: F, t532: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6829, t6834, t6835) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1224::<F>(t265, t394, t202, t6665, t1877, t1915, t193, t2522, t6670, t776, t868, t870, t1068, t1070, t336, t4700, t6818, t6822);
        let (t6840, t6841, t6848, t6855) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1225::<F>(t25, t1965, t40, t607, t6678, t6835, t28, t776, t868, t1081, t1877, t1915, t2522, t6666, t6670, dens_threshold, rho0, zeta_threshold);
        let (t6856, t6862) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1226::<F>(t28, t265, t504, t6834, t1972, t52, t607, t6855, t6840, dens_threshold, rho1, zeta_threshold);
        let (t6872, t6875, t6876) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1227::<F>(t1873, t2314, t5113, t1268, t6534, t6515, t6517, t671, t1271, t191, t192);
        let (t6877, t6878) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1228::<F>(t2020, t6876, t2018, t532);
    (t6829, t6835, t6841, t6848, t6856, t6862, t6872, t6875, t6876, t6877, t6878)
}
