//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta527(t225: f64, t387: f64, t4657: f64, t345: f64, t7569: f64, t1921: f64, t25749: f64, t986: f64, t7593: f64, t990: f64, t25705: f64, t349: f64, t1066: f64, t1920: f64, t23346: f64, t23385: f64, t23387: f64, t23389: f64, t3026: f64, t3169: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t6687: f64, t6771: f64, t6776: f64, t6816: f64, t7554: f64, t7566: f64, t7600: f64, t7625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25766, t25767, t25778, t25784, t25785, t25789, t25791) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1936(t225, t387, t4657, t345, t7569, t1921, t25749, t986, t7593, t990, t25705, t349);
        let t25794 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1937(t1066, t1920, t23346, t23385, t23387, t23389, t25767, t25778, t25785, t25789, t25791, t3026, t3169, t388, t4557, t4660, t4665, t6687, t6771, t6776, t6816, t7554, t7566, t7600, t7625);
    (t25766, t25767, t25778, t25784, t25785, t25789, t25791, t25794)
}
