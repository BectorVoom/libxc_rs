//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta870 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2768;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta870(t13999: f64, t22271: f64, t48919: f64, t6869: f64, t9816: f64, t9818: f64, t13847: f64, t22016: f64, t48731: f64, t73731: f64, t1399: f64, t73856: f64, t22298: f64, t48100: f64, t22129: f64, t2713: f64, t3964: f64, t22169: f64, t46691: f64, t22173: f64, t9744: f64, t6856: f64, t9779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74186, t74206, t74232, t74249) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2768(t13999, t22271, t48919, t6869, t9816, t9818, t13847, t22016, t48731, t73731, t1399, t73856);
        let (t74257, t74264, t74269, t74271, t74277) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2769(t22298, t48100, t9816, t22129, t2713, t3964, t22169, t46691, t22173, t9744, t6856, t9779);
    (t74186, t74206, t74232, t74249, t74257, t74264, t74269, t74271, t74277)
}
