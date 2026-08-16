//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 949/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk949(t7025: f64, t9239: f64, t33: f64, t625: f64, t2240: f64, t6492: f64, t2031: f64, t22550: f64, t6495: f64, t7032: f64, t9231: f64, t6486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23963 = t9239 * t7025;
    let t23966 = t33 * t625;
    let t23967 = t2240 * t23966;
    let t23968 = t23967 * t6492;
    let t23970 = t2031 * t22550;
    let t23973 = t6495 * t7032;
    let t23975 = t9231 * t7025;
    let t23978 = t6486 * t7032;
    (t23963, t23967, t23968, t23970, t23973, t23975, t23978)
}
