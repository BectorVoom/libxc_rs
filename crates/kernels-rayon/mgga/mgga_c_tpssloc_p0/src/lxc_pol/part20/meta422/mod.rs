//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1834;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1835;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1836;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta422(t13550: f64, t13563: f64, t10296: f64, t10298: f64, t10302: f64, t13566: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13644: f64, t13602: f64, t13598: f64, t13613: f64, t13630: f64, t13632: f64, t13635: f64, t13638: f64, t13640: f64, t13642: f64, t13647: f64, t10300: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10784: f64, t10785: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13552: f64, t13557: f64, t13561: f64, t13616: f64, t13624: f64, t13626: f64, t932: f64, t4446: f64, t942: f64, t1573: f64, t2929: f64, t13716: f64, t951: f64, t10608: f64, t324: f64, t2924: f64, t4475: f64, t10632: f64, t1580: f64, t2906: f64, t10756: f64, t10820: f64, t13729: f64, t14257: f64, t1581: f64, t2856: f64, t2900: f64, t2925: f64, t2930: f64, t2933: f64, t4434: f64, t4449: f64, t4472: f64, t924: f64, t943: f64, t952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14287, t14291, t14304) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1834(t13550, t13563, t10296, t10298, t10302, t13566, t13569, t13572, t13575, t13578, t13581, t13584, t13587);
        let t14328 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1835(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10556, t10558, t10560, t10562, t10784, t10785, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t14287, t14291, t14304);
        let (t14329, t14332, t14337, t14344, t14363) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1836(t14328, t932, t4446, t942, t1573, t2929, t13716, t951, t13563, t13566, t13602, t10556, t10558, t10560, t10562, t10608, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
        let (t14364, t14366, t14369, t14370, t14373) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1837(t14363, t324, t2924, t4475, t10632, t1580, t2906, t10756, t10820, t13729, t14257, t14329, t14332, t14337, t14344, t1581, t2856, t2900, t2925, t2930, t2933, t4434, t4449, t4472, t924, t943, t952);
    (t14328, t14329, t14332, t14337, t14344, t14363, t14364, t14366, t14369, t14370, t14373)
}
