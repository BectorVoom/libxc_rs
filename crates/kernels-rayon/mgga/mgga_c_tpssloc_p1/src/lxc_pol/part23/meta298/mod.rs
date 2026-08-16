//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1024;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta298(t21444: f64, t340: f64, t343: f64, t974: f64, t1597: f64, t5836: f64, t4546: f64, t5842: f64, t20217: f64, t978: f64, t977: f64, t10217: f64, t20234: f64, t10214: f64, t2980: f64, t21126: f64, t4518: f64, t13909: f64, t17784: f64, t17809: f64, t21430: f64, t21433: f64, t2986: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21446, t21447, t21452, t21453, t21456, t21458, t21459, t21462, t21463, t21468) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1024(t21444, t340, t343, t974, t1597, t5836, t4546, t5842, t20217, t978, t977, t10217, t20234);
        let (t21472, t21479) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1025(t10214, t21468, t20234, t2980, t977, t21126, t4518, t13909, t17784, t17809, t21430, t21433, t21447, t21453, t21459, t21463, t2986, t973);
    (t21446, t21452, t21456, t21458, t21462, t21468, t21472, t21479)
}
