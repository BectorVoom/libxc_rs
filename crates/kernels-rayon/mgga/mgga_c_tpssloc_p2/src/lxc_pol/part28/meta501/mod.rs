//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1733;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1734;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1735;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta501(t24175: f64, t7687: f64, t6999: f64, t7940: f64, t532: f64, t7939: f64, t6879: f64, t12571: f64, t7025: f64, t23967: f64, t7432: f64, t7032: f64, t7435: f64, t2032: f64, t23975: f64, t26055: f64, t26063: f64, t26067: f64, t26070: f64, t26073: f64, t26076: f64, t26090: f64, t6492: f64, t6495: f64, t7026: f64, t7035: f64, t7782: f64, t2031: f64, t26024: f64, t7428: f64, t26012: f64, t7031: f64, t7445: f64, t1860: f64, t22549: f64, t23963: f64, t23968: f64, t23970: f64, t23973: f64, t23978: f64, t23995: f64, t23999: f64, t26009: f64, t26016: f64, t26028: f64, t6486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26898, t26902, t26905, t26906, t26911, t26920, t26936) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1733(t24175, t7687, t6999, t7940, t532, t7939, t6879, t12571, t7025, t23967, t7432, t7032, t7435);
        let t26938 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1734(t2032, t23975, t26055, t26063, t26067, t26070, t26073, t26076, t26090, t26911, t26920, t26936, t6492, t6495, t7026, t7035, t7432, t7435, t7782);
        let (t26945, t26954, t26959, t26964) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1735(t2031, t26024, t7032, t7428, t26012, t7031, t7445, t1860, t2032, t22549, t23963, t23968, t23970, t23973, t23978, t23995, t23999, t26009, t26016, t26028, t6486, t7035, t7782);
    (t26898, t26902, t26905, t26906, t26911, t26938, t26945, t26954, t26959, t26964)
}
