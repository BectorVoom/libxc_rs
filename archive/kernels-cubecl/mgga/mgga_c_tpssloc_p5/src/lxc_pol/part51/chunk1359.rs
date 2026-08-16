//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1359/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1359<F: Float>(t1873: F, t4025: F, t2040: F, t33334: F, t532: F, t1983: F, t6879: F, t33234: F, t6535: F, t23938: F, t7461: F, t26977: F) -> (F, F, F, F, F, F) {
    let t120952 = t4025 * t1873;
    let t120954 = F::cast_from(2.0_f64) * t120952 * t2040;
    let t120955 = t532 * t33334;
    let t120958 = F::cast_from(3.0_f64) * t1983 * t120955 * t6879;
    let t120962 = F::cast_from(2.0_f64) * t33234 * t6535;
    let t120964 = F::cast_from(2.0_f64) * t23938 * t7461;
    let t120966 = F::cast_from(2.0_f64) * t26977 * t7461;
    (t120952, t120954, t120958, t120962, t120964, t120966)
}
