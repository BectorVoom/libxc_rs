//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1964;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta431(t15320: f64, t3451: f64, t11579: f64, t4919: f64, t11584: f64, t1174: f64, t15294: f64, t15300: f64, t15304: f64, t15307: f64, t15314: f64, t15317: f64, t3443: f64, t3447: f64, t3457: f64, t3461: f64, t4889: f64, t14753: f64, t4908: f64, t14744: f64, t11588: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15321, t15324, t15327, t15330) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1964(t15320, t3451, t11579, t4919, t11584, t1174, t15294, t15300, t15304, t15307, t15314, t15317, t3443, t3447, t3457, t3461, t4889);
        let (t15332, t15335, t15338) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1965(t14753, t4908, t14744, t11588, t1714);
    (t15321, t15324, t15327, t15330, t15332, t15335, t15338)
}
