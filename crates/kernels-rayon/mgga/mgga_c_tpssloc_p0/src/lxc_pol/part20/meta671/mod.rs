//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2521;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2522;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2523;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2524;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta671(t11270: f64, t4740: f64, t11274: f64, t1657: f64, t11278: f64, t1671: f64, t43954: f64, t11180: f64, t4782: f64, t14914: f64, t3259: f64, t1254: f64, t15834: f64, t3640: f64, t4700: f64, t50816: f64, t50818: f64, t50821: f64, t51111: f64, t51113: f64, t11131: f64, t4869: f64, t11427: f64, t14850: f64, t50826: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50824: f64, t50828: f64, t50832: f64, t50834: f64, t50837: f64, t50839: f64, t50853: f64, t43768: f64, t43770: f64, t44027: f64, t50846: f64, t50848: f64, t50851: f64, t50859: f64, t50863: f64, t50867: f64, t50871: f64, t50875: f64, t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t50881: f64, t50886: f64, t50897: f64, t50900: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50919: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64, t50937: f64, t50940: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51119, t51122, t51124, t51126, t51128, t51129) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2521(t11270, t4740, t11274, t1657, t11278, t1671, t43954, t11180, t4782, t14914, t3259, t1254, t15834, t3640, t4700, t50816, t50818, t50821, t51111, t51113);
        let (t51131, t51133, t51147) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2522(t11131, t4869, t11427, t14850, t50826, t43727, t43729, t43748, t43750, t50824, t50828, t50832, t50834, t50837, t50839);
        let t51159 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2523(t50853, t43768, t43770, t44027, t50846, t50848, t50851, t50859, t50863, t50867, t50871, t50875);
        let t51173 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2524(t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863, t50881, t50886, t50897, t50900);
        let t51186 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2525(t50903, t50905, t50907, t50912, t50917, t50919, t50921, t50926, t50931, t50934, t50937, t50940);
    (t51119, t51122, t51124, t51126, t51128, t51129, t51131, t51133, t51147, t51159, t51173, t51186)
}
