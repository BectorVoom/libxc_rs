//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1225;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta410(t11789: f64, t1227: f64, t248: f64, t5975: f64, t15437: f64, t15502: f64, t15506: f64, t19201: f64, t3576: f64, t3577: f64, t44951: f64, t6191: f64, t15568: f64, t5064: f64, t45046: f64, t5971: f64, t3032: f64, t65253: f64, t3505: f64, t3514: f64, t1174: f64, t6187: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65689, t65703, t65706, t65815, t65819) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1225(t11789, t1227, t248, t5975, t15437, t15502, t15506, t19201, t3576, t3577, t44951, t6191);
        let (t65884, t65935, t65963, t65966, t66015) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1226(t15568, t5064, t1227, t248, t45046, t5971, t3032, t65253, t3505, t3514, t1174, t6187, t698);
    (t65689, t65703, t65706, t65815, t65819, t65884, t65935, t65963, t65966, t66015)
}
