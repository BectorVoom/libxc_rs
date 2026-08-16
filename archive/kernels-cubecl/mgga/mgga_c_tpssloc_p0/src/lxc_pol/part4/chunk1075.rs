//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1075/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1075<F: Float>(t17817: F, t2988: F, t17183: F, t4518: F, t135: F, t5844: F, t973: F, t10295: F, t10296: F, t13642: F, t13921: F, t13922: F, t13923: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F) -> (F, F, F, F) {
    let t17818 = t2988 * t17817;
    let t17821 = t4518 * t17183;
    let t17826 = t135 * t5844;
    let t17827 = t973 * t17826;
    let t17841 = t10295 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t10296 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t13642 - t13921 + t13922 - t13923 - t17286 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t17244 - t17280 / F::cast_from(3.0_f64) + t17241 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17288 + t17247 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t17250 - t17290 / F::cast_from(9.0_f64) + t17256 / F::cast_from(18.0_f64) - t17253 / F::cast_from(3.0_f64) + t17293 / F::cast_from(6.0_f64);
    (t17818, t17821, t17827, t17841)
}
