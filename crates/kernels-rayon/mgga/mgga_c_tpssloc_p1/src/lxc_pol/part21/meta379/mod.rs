//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1835;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1836;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta379(t13851: f64, t2990: f64, t10245: f64, t4531: f64, t10241: f64, t4514: f64, t2989: f64, t3966: f64, t2988: f64, t13542: f64, t4518: f64, t13546: f64, t10259: f64, t13559: f64, t13555: f64, t4510: f64, t1597: f64, t3014: f64, t343: f64, t4546: f64, t3008: f64, t2960: f64, t4506: f64, t10263: f64, t13850: f64, t1593: f64, t2986: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
        let (t13852, t13855, t13858, t13861) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1835(t13851, t2990, t10245, t4531, t10241, t4514, t2989, t3966);
        let (t13862, t13865, t13868, t13871, t13874, t13877, t13881) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1836(t13861, t2988, t13542, t4518, t13546, t10259, t4514, t13559, t13555, t4510, t1597, t3014, t343);
        let (t13886, t13893, t13894) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1837(t13881, t4546, t1597, t3008, t343, t2960, t4506, t10263, t13850, t13852, t13855, t13858, t13862, t13865, t13868, t13871, t13874, t13877, t1593, t2986, t973);
    (t13861, t13881, t13886, t13893, t13894)
}
