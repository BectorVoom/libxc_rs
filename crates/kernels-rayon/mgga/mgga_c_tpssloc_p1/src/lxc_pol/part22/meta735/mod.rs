//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2413;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2414;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta735(t21238: f64, t2929: f64, t4497: f64, t959: f64, t17934: f64, t4489: f64, t4498: f64, t17565: f64, t21089: f64, t41825: f64, t17951: f64, t4483: f64, t17566: f64, t4475: f64, t60963: f64, t21334: f64, t892: f64, t914: f64, t1580: f64, t49513: f64, t60722: f64, t950: f64, t1637: f64, t17198: f64, t4696: f64, t4700: f64, t60867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68905, t68910, t68912, t68916, t68918) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2413(t21238, t2929, t4497, t959, t17934, t4489, t4498, t17565, t21089, t41825, t17951, t4483);
        let (t68920, t68923, t68926, t68930) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2414(t17566, t4483, t4475, t60963, t959, t21334, t892, t914, t1580, t49513, t60722, t950);
        let t68931 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2415(t1637, t17198, t4696, t4700, t60867, t68905, t68910, t68912, t68916, t68918, t68920, t68923, t68926, t68930);
    (t68905, t68910, t68912, t68916, t68918, t68920, t68923, t68926, t68930, t68931)
}
