//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1351/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1351(t13798: f64, t17794: f64, t17800: f64, t17804: f64, t17817: f64, t17863: f64, t2986: f64, t42817: f64, t4510: f64, t4514: f64, t4518: f64, t4531: f64, t48221: f64, t61322: f64, t69496: f64, t69505: f64, t69519: f64, t69529: f64, t69570: f64, t69579: f64, t76585: f64, t76608: f64, t76616: f64, t76624: f64) -> f64 {
    let t76865 = -0.11111111111111111111e-2_f64 * t69570 + 0.99999999999999999996e-2_f64 * t2986 * t4518 * t76616 + 0.14814814814814814815e-2_f64 * t2986 * t4510 * t76608 + 0.51851851851851851851e-2_f64 * t2986 * t13798 * t76585 - 0.22222222222222222222e-2_f64 * t2986 * t61322 * t17863 - 0.34567901234567901234e-2_f64 * t2986 * t48221 * t69519 - 0.11111111111111111111e-2_f64 * t2986 * t69496 * t4514 - 0.16666666666666666666e-2_f64 * t2986 * t17800 * t17794 - 0.11111111111111111111e-2_f64 * t2986 * t69505 * t4514 - 0.66666666666666666664e-2_f64 * t2986 * t4531 * t69529 + 0.33333333333333333332e-2_f64 * t2986 * t17804 * t17817 - t42817 - 0.11111111111111111111e-2_f64 * t69579 - 0.22222222222222222221e-2_f64 * t2986 * t4518 * t76624;
    t76865
}
