//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2109;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta593(t47093: f64, t4159: f64, t9541: f64, t1516: f64, t41052: f64, t4166: f64, t9600: f64, t849: f64, t13176: f64, t2696: f64, t1509: f64, t9975: f64, t242: f64, t41347: f64, t812: f64, t2627: f64, t4265: f64, t226: f64, t40931: f64, t68: f64, t2394: f64, t4344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47094, t47231, t47270, t47275, t47277, t47278, t47285) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2109(t47093, t4159, t9541, t1516, t41052, t4166, t9600, t849, t13176, t2696, t1509, t9975);
        let (t47307, t47374, t47386, t47705) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2110(t242, t41347, t812, t2627, t4265, t226, t40931, t68, t2394, t4344);
    (t47094, t47231, t47270, t47275, t47277, t47278, t47285, t47307, t47374, t47386, t47705)
}
