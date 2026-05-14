//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1413/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1413<F: Float>(t1306: F, t135: F, t2457: F, t2461: F, t2464: F, t26883: F, t26885: F, t26888: F, t26890: F, t26892: F, t26895: F, t26898: F, t26900: F, t26901: F, t26905: F, t27253: F, t273: F, t27398: F, t28536: F, t28569: F, t957: F, t9759: F) -> (F,) {
    let t28576 = t26883 + t26885 + t26888 - t26890 - t26892 - t26895 + t26898 - t26900 + 2.0 * t1306 * t26901 * t2461 - 2.0 * t135 * t273 * t26905 * t2464 + t135 * t273 * (t28536 + t28569) * t957 - t1306 * t9759 * t2457 + t27253 - t27398;
    (t28576,)
}
