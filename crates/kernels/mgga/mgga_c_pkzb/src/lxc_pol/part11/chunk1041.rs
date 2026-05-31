//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1041/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1041<F: Float>(t1259: F, t3936: F, t11240: F, t11242: F, t11245: F, t11263: F, t11266: F, t11292: F, t11295: F, t11357: F, t11359: F, t11361: F, t11367: F, t1306: F, t135: F, t273: F, t6362: F, t9759: F) -> (F, F) {
    let t11541 = t3936 * t1259;
    let t11549 = F::cast_from(2.0_f64) * t11541 * t135 * t273 * t6362 - F::cast_from(3.0_f64) * t1259 * t1306 * t9759 + t11240 + t11242 - t11245 + t11263 + t11266 - t11292 + t11295 - t11357 - t11359 - t11361 - t11367;
    (t11541, t11549)
}
