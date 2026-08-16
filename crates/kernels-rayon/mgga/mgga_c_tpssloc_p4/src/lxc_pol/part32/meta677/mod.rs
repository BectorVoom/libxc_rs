//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2114;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta677(t24740: f64, t5064: f64, t15640: f64, t24729: f64, t24574: f64, t27574: f64, t24844: f64, t7999: f64, t2121: f64, t3427: f64, t8077: f64, t27517: f64, t85639: f64, t27481: f64, t7365: f64, t94490: f64, t1715: f64, t974: f64, t24847: f64, t24771: f64, t15418: f64, t2127: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95687, t95702, t95714, t95722, t95726, t95747) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2114(t24740, t5064, t15640, t24729, t24574, t27574, t24844, t7999, t2121, t3427, t8077, t27517, t85639);
        let (t95751, t95758, t95760, t95761, t95768, t95772) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2115(t24574, t27481, t7365, t94490, t1715, t974, t24847, t24771, t7999, t15418, t2127, t221);
    (t95687, t95702, t95714, t95722, t95726, t95747, t95751, t95758, t95760, t95761, t95768, t95772)
}
