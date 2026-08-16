//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2557/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2557<F: Float>(t1164: F, t44106: F, t4882: F, t14842: F, t3411: F, t11940: F, t4700: F, t5095: F, t51131: F, t51133: F, t51245: F, t51248: F, t51251: F, t51793: F, t51795: F, t51797: F) -> (F, F, F) {
    let t51800 = F::cast_from(0.17315859105681463759e2_f64) * t1164 * t4882 * t44106;
    let t51802 = F::cast_from(0.31168546390226634765e3_f64) * t3411 * t14842;
    let t51803 = -t11940 * t4700 * t5095 - t51131 + t51133 + t51245 - t51248 - t51251 + t51793 - t51795 - t51797 - t51800 + t51802;
    (t51800, t51802, t51803)
}
