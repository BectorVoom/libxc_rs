//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1208/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1208(t1880: f64, t32875: f64, t25: f64, t7540: f64, t28: f64, t1458: f64, t1868: f64) -> (f64, f64, f64, f64) {
    let t32877 = 0.16449340668482264365e-1_f64 * t1880 * t32875;
    let t32899 = t25 * t7540;
    let t33065 = t28 * t7540;
    let t33085 = t1868 * t1458;
    (t32877, t32899, t33065, t33085)
}
