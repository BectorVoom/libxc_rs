//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1707/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1707(t103: f64, t584: f64, t16: f64, t4063: f64, t100: f64, t12771: f64, t12774: f64, t12775: f64, t12778: f64, t12781: f64, t12784: f64, t12792: f64, t12795: f64, t12796: f64, t12799: f64, t1445: f64, t1447: f64, t2336: f64, t2351: f64, t2355: f64, t4050: f64, t4054: f64, t657: f64, t92: f64) -> f64 {
    let t12802 = t103 * t584;
    let t12805 = t4063 * t16;
    let t12808 = 200.0_f64 / 27.0_f64 * t2336 * t1445 - 100.0_f64 / 27.0_f64 * t657 * t4050 - 50.0_f64 / 9.0_f64 * t657 * t4054 - 10.0_f64 / 27.0_f64 * t92 * t12771 + 20.0_f64 / 9.0_f64 * t12774 * t12775 + 10.0_f64 / 9.0_f64 * t92 * t12778 + 5.0_f64 / 3.0_f64 * t92 * t12781 - 5.0_f64 * t92 * t12784 - 50.0_f64 / 27.0_f64 * t1447 * t2351 - 25.0_f64 / 9.0_f64 * t1447 * t2355 - 10.0_f64 / 27.0_f64 * t100 * t12792 - 20.0_f64 / 9.0_f64 * t12795 * t12796 + 10.0_f64 / 9.0_f64 * t100 * t12799 - 5.0_f64 / 3.0_f64 * t100 * t12802 + 5.0_f64 * t100 * t12805;
    t12808
}
