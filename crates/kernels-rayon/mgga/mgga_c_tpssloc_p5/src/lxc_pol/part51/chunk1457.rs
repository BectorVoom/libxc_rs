//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1457/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1457(t115774: f64, t1983: f64, t7687: f64, t1307: f64, t22574: f64, t26558: f64, t33221: f64, t122645: f64, t122656: f64, t122659: f64, t122662: f64, t122664: f64, t1393: f64, t1976: f64, t22461: f64, t26103: f64, t26880: f64, t26967: f64, t27163: f64, t33085: f64, t33601: f64, t6517: f64, t7057: f64, t7796: f64, t8450: f64) -> f64 {
    let t122667 = 3.0_f64 * t1983 * t115774 * t7687;
    let t122671 = 6.0_f64 * t22574 * t26558 * t33221 * t1307;
    let t122673 = t1393 * t33601 - t1976 * t26967 - 2.0_f64 * t22461 * t7796 - 2.0_f64 * t26103 * t7796 - t26880 * t8450 - 2.0_f64 * t27163 * t6517 - 2.0_f64 * t33085 * t7057 - t122645 + t122656 - t122659 - t122662 - t122664 + t122667 + t122671;
    t122673
}
