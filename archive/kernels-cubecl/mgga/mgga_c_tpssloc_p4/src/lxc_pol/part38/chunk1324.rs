//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1324/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1324<F: Float>(t110075: F, t30149: F, t29895: F, t30156: F, t110082: F, t110148: F, t110150: F, t1444: F, t2176: F, t2248: F, t2585: F, t29903: F, t29922: F, t29926: F, t30155: F, t4067: F, t659: F, t8128: F, t8129: F, t8137: F, t8138: F, t86592: F, t86595: F, t86598: F, t95: F) -> F {
    let t110564 = F::cast_from(4.0_f64) * t110075 * t30149;
    let t110566 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t29895 * t30156;
    let t110580 = -F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t110148 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t110150 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t2585 * t2176 * t95 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8137 * t29926 * t1444 * t2248 + F::cast_from(3.0_f64) * t110082 * t8129 * t86592 + t110564 - t110566 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t29903 * t8129 * t86595 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t29903 * t8129 * t86598 - F::cast_from(25.0_f64) / F::cast_from(18.0_f64) * t8128 * t29922 * t30155 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8128 * t8138 * t4067 * t659;
    t110580
}
