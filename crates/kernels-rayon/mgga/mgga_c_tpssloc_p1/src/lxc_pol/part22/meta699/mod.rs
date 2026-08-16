//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2282;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta699(t1222: f64, t18574: f64, t11789: f64, t1227: f64, t248: f64, t5975: f64, t18321: f64, t3548: f64, t15437: f64, t15502: f64, t15506: f64, t4965: f64, t5023: f64, t15643: f64, t5024: f64, t19201: f64, t3576: f64, t3577: f64, t44951: f64, t6191: f64, t13969: f64, t19061: f64, t3515: f64, t15568: f64, t5064: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65681, t65689, t65691, t65703, t65706, t65709) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2282(t1222, t18574, t11789, t1227, t248, t5975, t18321, t3548, t15437, t15502, t15506, t4965, t5023);
        let (t65803, t65815, t65819, t65881, t65884) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2283(t15643, t5024, t19201, t3576, t3577, t44951, t6191, t13969, t19061, t3515, t15568, t5064);
    (t65681, t65689, t65691, t65703, t65706, t65709, t65803, t65815, t65819, t65881, t65884)
}
