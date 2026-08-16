//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2132;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta570(t2986: f64, t2990: f64, t42771: f64, t10346: f64, t2987: f64, t10190: f64, t10245: f64, t10250: f64, t13779: f64, t10255: f64, t2989: f64, t9258: f64, t10337: f64, t964: f64, t340: f64, t625: f64, t221: f64, t339: f64, t344: f64, t10195: f64, t13784: f64, t1887: f64, t2262: f64, t337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42773, t42775, t42785, t42788, t42794, t42799) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2132(t2986, t2990, t42771, t10346, t2987, t10190, t10245, t10250, t13779, t10255, t2989, t9258);
        let (t42811, t42813, t42817, t42827, t42830) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2133(t10337, t964, t340, t625, t221, t339, t344, t10195, t13784, t2986, t1887, t2262, t337);
    (t42773, t42775, t42785, t42788, t42794, t42799, t42811, t42813, t42817, t42827, t42830)
}
