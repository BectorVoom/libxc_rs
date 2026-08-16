//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1868;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1869;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta487(t28: f64, t265: f64, t504: f64, t23772: f64, t1972: f64, t2250: f64, t23820: f64, t52: f64, t607: f64, t6856: f64, t23780: f64, t1873: f64, t3652: f64, t652: f64, t6876: f64, t7000: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t6880: f64, t9348: f64, t12734: f64, t2314: f64, t6534: f64, t12739: f64, t5113: f64, t1268: f64, t22479: f64, t22461: f64, t22559: f64, t22600: f64, t2363: f64, t6517: f64, t671: f64, t12461: f64, t3698: f64, t2019: f64, t1983: f64, t113: f64, t1976: f64, t22594: f64, t22599: f64, t22605: f64, t22608: f64, t22610: f64, t22612: f64, t22614: f64, t22616: f64, t22618: f64, t22619: f64, t22950: f64, t2312: f64, t2364: f64, t510: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23821, t23829, t23831, t23833, t23835) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1868(t28, t265, t504, t23772, t1972, t2250, t23820, t52, t607, t6856, t23780, t1873, t3652, t652, t6876, t7000, dens_threshold, rho1, zeta_threshold);
        let (t23837, t23855) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1869(t6876, t6880, t1873, t9348, t12734, t2314, t6534, t12739, t5113, t1268, t22479, t22461, t22559, t22600, t2363, t6517, t671);
        let (t23857, t23858, t23861) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1870(t12461, t3698, t2019, t1983, t113, t1976, t22594, t22599, t22600, t22605, t22608, t22610, t22612, t22614, t22616, t22618, t22619, t22950, t2312, t2364, t23829, t23833, t23835, t23837, t23855, t510, t574, t6517, t652);
    (t23821, t23829, t23831, t23855, t23857, t23858, t23861)
}
