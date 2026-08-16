//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 760/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk760<F: Float>(t1799: F, t1824: F, t550: F, t1339: F, t22827: F, t22833: F, t6396: F, t22820: F, t22826: F, t22859: F, t22864: F, t22868: F, t26272: F, t26295: F, t28085: F, t28089: F, t28091: F, t28093: F, t28095: F, t28097: F) -> (F, F, F, F) {
    let t28099 = t1799 * t1824;
    let t28100 = t28099 * t550;
    let t28101 = t1339 * t28100;
    let t28102 = t22827 * t28101;
    let t28104 = t22833 * t6396;
    let t28106 = F::cast_from(0.40372756094140390854e-3_f64) * t26272 + t28085 / F::cast_from(768.0_f64) - t22820 + t22826 + F::cast_from(0.28260929265898273598e-2_f64) * t26295 + t28089 / F::cast_from(1536.0_f64) - t28091 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t28093 - t28095 / F::cast_from(384.0_f64) - t28097 / F::cast_from(192.0_f64) + F::cast_from(0.24223653656484234512e-2_f64) * t28102 + t22859 + t22864 + t22868 + t28104 / F::cast_from(192.0_f64);
    (t28100, t28102, t28104, t28106)
}
