//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2557/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2557(t1164: f64, t44106: f64, t4882: f64, t14842: f64, t3411: f64, t11940: f64, t4700: f64, t5095: f64, t51131: f64, t51133: f64, t51245: f64, t51248: f64, t51251: f64, t51793: f64, t51795: f64, t51797: f64) -> (f64, f64, f64) {
    let t51800 = 0.17315859105681463759e2_f64 * t1164 * t4882 * t44106;
    let t51802 = 0.31168546390226634765e3_f64 * t3411 * t14842;
    let t51803 = -t11940 * t4700 * t5095 - t51131 + t51133 + t51245 - t51248 - t51251 + t51793 - t51795 - t51797 - t51800 + t51802;
    (t51800, t51802, t51803)
}
