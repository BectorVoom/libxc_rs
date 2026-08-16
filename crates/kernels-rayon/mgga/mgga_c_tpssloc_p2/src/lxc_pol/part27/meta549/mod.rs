//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1982;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1983;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta549(t26347: f64, t3887: f64, t22666: f64, t7700: f64, t1985: f64, t1842: f64, t3886: f64, t1385: f64, t22635: f64, t1992: f64, t6883: f64, t7697: f64, t1375: f64, t16460: f64, t2016: f64, t26224: f64, t26226: f64, t26229: f64, t26329: f64, t26335: f64, t26340: f64, t26345: f64, t3882: f64, t5321: f64, t568: f64, t6963: f64, t7729: f64, t225: f64, t7723: f64, t2015: f64, t5353: f64, t22897: f64, t5336: f64, t22751: f64, t7733: f64, t1799: f64, t22881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26348, t26351, t26352, t26354, t26355, t26356, t26357, t26361) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1982(t26347, t3887, t22666, t7700, t1985, t1842, t3886, t1385, t22635, t1992, t6883, t7697);
        let t26364 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1983(t1375, t16460, t2016, t26224, t26226, t26229, t26329, t26335, t26340, t26345, t26348, t26352, t26357, t26361, t3882, t5321, t568, t6963, t7729);
        let (t26366, t26371, t26378, t26379, t26381, t26384) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1984(t225, t7723, t2015, t5353, t3887, t22897, t5336, t1992, t22751, t7733, t1799, t22881);
    (t26348, t26351, t26354, t26355, t26356, t26364, t26366, t26371, t26378, t26379, t26381, t26384)
}
