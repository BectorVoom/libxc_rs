//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1822;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1823;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta466(t225: f64, t3166: f64, t387: f64, t345: f64, t1922: f64, t2966: f64, t1920: f64, t1049: f64, t6703: f64, t6706: f64, t6710: f64, t6769: f64, t1955: f64, t3206: f64, t3174: f64, t10160: f64, t1052: f64, t1066: f64, t1956: f64, t23346: f64, t3169: f64, t3176: f64, t3207: f64, t6687: f64, t6695: f64, t6771: f64, t6816: f64, t134: f64, t221: f64, t1926: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23353, t23354, t23357, t23359, t23365, t23366, t23369, t23372) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1822(t225, t3166, t387, t345, t1922, t2966, t1920, t1049, t6703, t6706, t6710, t6769);
        let (t23378, t23381) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1823(t1955, t3206, t3174, t10160, t1052, t1066, t1920, t1956, t23346, t23354, t23359, t23366, t23369, t23372, t3169, t3176, t3207, t6687, t6695, t6771, t6816);
        let (t23383, t23384) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1824(t134, t221, t1926);
    (t23353, t23357, t23359, t23365, t23366, t23369, t23372, t23378, t23381, t23383, t23384)
}
