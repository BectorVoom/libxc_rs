//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1224;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1225;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1226;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1227;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta252(t265: f64, t394: f64, t202: f64, t6665: f64, t1877: f64, t1915: f64, t193: f64, t2522: f64, t6670: f64, t776: f64, t868: f64, t870: f64, t1068: f64, t1070: f64, t336: f64, t4700: f64, t6818: f64, t6822: f64, t25: f64, t1965: f64, t40: f64, t607: f64, t6678: f64, t28: f64, t1081: f64, t6666: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t504: f64, t1972: f64, t52: f64, rho1: f64, t1873: f64, t2314: f64, t5113: f64, t1268: f64, t6534: f64, t6515: f64, t6517: f64, t671: f64, t1271: f64, t191: f64, t192: f64, t2020: f64, t2018: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6829, t6834, t6835) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1224(t265, t394, t202, t6665, t1877, t1915, t193, t2522, t6670, t776, t868, t870, t1068, t1070, t336, t4700, t6818, t6822);
        let (t6840, t6841, t6848, t6855) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1225(t25, t1965, t40, t607, t6678, t6835, t28, t776, t868, t1081, t1877, t1915, t2522, t6666, t6670, dens_threshold, rho0, zeta_threshold);
        let (t6856, t6862) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1226(t28, t265, t504, t6834, t1972, t52, t607, t6855, t6840, dens_threshold, rho1, zeta_threshold);
        let (t6872, t6875, t6876) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1227(t1873, t2314, t5113, t1268, t6534, t6515, t6517, t671, t1271, t191, t192);
        let (t6877, t6878) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1228(t2020, t6876, t2018, t532);
    (t6829, t6835, t6841, t6848, t6856, t6862, t6872, t6875, t6876, t6877, t6878)
}
