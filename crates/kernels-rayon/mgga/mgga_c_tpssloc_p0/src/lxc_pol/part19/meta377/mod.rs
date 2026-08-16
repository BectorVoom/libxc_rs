//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1409;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1410;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta377(t1113: f64, t136: f64, t43800: f64, t43804: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43777: f64, t43833: f64, t43835: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64, t2403: f64, t3298: f64, t11220: f64, t699: f64, t1114: f64, t9709: f64, t3304: f64, t3301: f64, t1102: f64, t11258: f64, t3270: f64, t3287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43848, t43851, t43853) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1409(t1113, t136, t43800, t43804, t43759, t43766, t43768, t43770, t43773, t43777, t43833, t43835, t43837, t43839, t43842, t43845);
        let (t43855, t43857, t43859, t43861, t43863, t43866, t43869) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1410(t2403, t3298, t11220, t699, t1114, t9709, t3304, t3301, t1102, t11258, t3270, t3287);
    (t43848, t43851, t43853, t43855, t43857, t43859, t43861, t43863, t43866, t43869)
}
