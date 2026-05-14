//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1000/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1000<F: Float>(t34265: F, t6963: F, t15128: F, t34230: F, t25188: F, t28854: F, t34089: F, t10697: F, t142618: F, t1466: F, t193: F, t28870: F, t29008: F, t29033: F, t29047: F, t29416: F, t33966: F, t34003: F, t34254: F, t34312: F, t36068: F, t7028: F, t7581: F, t7587: F, t875: F, t99918: F) -> (F, F, F, F) {
    let t153493 = t6963 * t34265;
    let t153507 = t15128 * t34230;
    let t153509 = t25188 * t28854;
    let t153511 = t15128 * t34089;
    let t153520 = -t153493 / 18.0 + t1466 * t193 * t33966 * t29033 + t142618 / 9.0 - t6963 * t34254 / 3.0 - t7581 * t28870 / 3.0 + t34312 * t7028 / 6.0 - t29008 * t34003 / 18.0 + 4.0 * t153507 + 8.0 * t153509 + 8.0 * t153511 - 24.0 * t99918 * t29047 - 24.0 * t10697 * t36068 * t875 - t29416 * t7587 / 3.0;
    (t153507, t153509, t153511, t153520)
}
