//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2283/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2283(t15643: f64, t5024: f64, t19201: f64, t3576: f64, t3577: f64, t44951: f64, t6191: f64, t13969: f64, t19061: f64, t3515: f64, t15568: f64, t5064: f64) -> (f64, f64, f64, f64, f64) {
    let t65803 = t5024 * t15643;
    let t65815 = t19201 * t3576;
    let t65819 = t3577 * t44951 * t6191;
    let t65881 = t3515 * t13969 * t19061;
    let t65884 = t5064 * t15568;
    (t65803, t65815, t65819, t65881, t65884)
}
