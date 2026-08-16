//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1088;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1089;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1090;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta260(t25: f64, t265: f64, t394: f64, t202: f64, t7109: f64, t1877: f64, t193: f64, t2057: f64, t2522: f64, t7114: f64, t776: f64, t868: f64, t870: f64, t2064: f64, t40: f64, t606: f64, t607: f64, t6542: f64, t6671: f64, t7110: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t28: f64, t504: f64, t1081: f64, t2071: f64, t52: f64, t6841: f64, t6848: f64, rho1: f64, t1268: f64, t2039: f64, t2314: f64, t5113: f64, t671: f64, t7040: f64, t7042: f64, t7056: f64, t2094: f64, t532: f64) -> (f64, f64, f64, f64, f64) {
        let (t7130, t7131, t7136) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1088(t25, t265, t394, t202, t7109, t1877, t193, t2057, t2522, t7114, t776, t868, t870, t2064, t40, t606, t607, t6542, t6671, t7110, dens_threshold, rho0, zeta_threshold);
        let (t7150, t7155) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1089(t28, t265, t504, t7130, t1081, t1877, t2057, t2071, t2522, t52, t607, t6841, t6848, t7110, t7114, dens_threshold, rho1, zeta_threshold);
        let t7156 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1090(t7136, t7155);
        let (t7166, t7170) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1091(t1268, t2039, t2314, t5113, t671, t7040, t7042, t7056, t2094, t532);
    (t7131, t7150, t7156, t7166, t7170)
}
