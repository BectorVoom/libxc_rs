//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2587;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta731(t3447: f64, t44583: f64, t461: f64, t4729: f64, t15418: f64, t1714: f64, t11571: f64, t14736: f64, t15419: f64, t14165: f64, t44505: f64, t11557: f64, t4889: f64, t11560: f64, t1174: f64, t1716: f64, t2402: f64, t4930: f64, t698: f64, t11513: f64, t11589: f64, t15313: f64, t14749: f64, t15402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52057, t52059, t52061, t52064, t52066, t52074) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2587(t3447, t44583, t461, t4729, t15418, t1714, t11571, t14736, t15419, t14165, t44505, t11557, t4889);
        let (t52076, t52081, t52084, t52086, t52089, t52092) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2588(t11560, t4889, t1174, t1716, t2402, t4930, t698, t11513, t11589, t15313, t3447, t14749, t15402);
    (t52057, t52059, t52061, t52064, t52066, t52074, t52076, t52081, t52084, t52086, t52089, t52092)
}
