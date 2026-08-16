//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1932;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1933;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta525(t25717: f64, t6784: f64, t2770: f64, t381: f64, t3961: f64, t25510: f64, t23613: f64, t7603: f64, t1003: f64, t1058: f64, t23327: f64, t23346: f64, t23712: f64, t25429: f64, t25563: f64, t25568: f64, t25706: f64, t25708: f64, t25714: f64, t3186: f64, t353: f64, t6680: f64, t6687: f64, t7604: f64, t7615: f64, t7622: f64, t25482: f64, t25527: f64, t25560: f64, t1055: f64, t23384: f64, t7566: f64, t23394: f64, t4664: f64, t6704: f64, t1634: f64, t6815: f64, t3174: f64, t1054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25718, t25721, t25722, t25723, t25726, t25729) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1932(t25717, t6784, t2770, t381, t3961, t25510, t23613, t7603, t1003, t1058, t23327, t23346, t23712, t25429, t25563, t25568, t25706, t25708, t25714, t3186, t353, t6680, t6687, t7604, t7615, t7622);
        let (t25731, t25732, t25736, t25738, t25739, t25742) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1933(t25482, t25527, t25560, t25729, t1055, t23384, t7566, t23394, t4664, t6704, t1634, t6815);
        let (t25743, t25749) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1934(t25742, t3174, t1054, t1634);
    (t25718, t25721, t25722, t25723, t25726, t25731, t25732, t25736, t25738, t25739, t25743, t25749)
}
