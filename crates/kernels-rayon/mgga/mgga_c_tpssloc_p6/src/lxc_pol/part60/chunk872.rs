//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 872/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk872(t1880: f64, t32875: f64, t25: f64, t7540: f64, t28: f64, t3701: f64, t7752: f64, t4028: f64, t8326: f64, t7676: f64, t1458: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32877 = 0.16449340668482264365e-1_f64 * t1880 * t32875;
    let t32899 = t25 * t7540;
    let t33065 = t28 * t7540;
    let t33136 = t3701 * t7752;
    let t33151 = t4028 * t8326;
    let t33152 = 2.0_f64 * t33151;
    let t33153 = t7676 * t8326;
    let t33154 = 2.0_f64 * t33153;
    let t33185 = t576 * t1458;
    (t32877, t32899, t33065, t33136, t33151, t33152, t33153, t33154, t33185)
}
