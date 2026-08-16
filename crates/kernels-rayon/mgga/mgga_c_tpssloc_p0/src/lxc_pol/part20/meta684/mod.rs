//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2592;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2593;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta684(t11712: f64, t11880: f64, t491: f64, t1734: f64, t6739: f64, t3609: f64, t52434: f64, t3507: f64, t5052: f64, t1215: f64, t2250: f64, t475: f64, t2244: f64, t3242: f64, t1216: f64, t3493: f64, t1011: f64, t1212: f64, t52446: f64, t11539: f64, t1174: f64, t14736: f64, t1227: f64, t13969: f64, t15544: f64, t15655: f64, t15636: f64, t3515: f64, t44571: f64, t4724: f64, t11778: f64, t43791: f64, t11720: f64, t11722: f64, t11748: f64, t15498: f64, t3587: f64, t44725: f64, t44811: f64, t44863: f64, t45030: f64, t4582: f64, t48497: f64, t4889: f64, t4977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52479, t52480, t52485, t52500, t52532) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2592(t11712, t11880, t491, t1734, t6739, t3609, t52434, t3507, t5052, t1215, t2250, t475);
        let (t52538, t52548, t52554, t52568, t52575) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2593(t1215, t2244, t475, t3242, t1216, t3493, t1011, t1212, t52446, t11539, t1174, t14736);
        let t52606 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2594(t1227, t13969, t15544, t15655, t15636, t3515, t1174, t44571, t4724, t11778, t43791, t11720, t11722, t11748, t15498, t3587, t44725, t44811, t44863, t45030, t4582, t48497, t4889, t4977, t52575);
    (t52479, t52480, t52485, t52500, t52532, t52538, t52548, t52554, t52568, t52606)
}
