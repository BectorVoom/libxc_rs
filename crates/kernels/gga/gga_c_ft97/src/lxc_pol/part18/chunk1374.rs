//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1374/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1374<F: Float>(t2101: F, t6685: F, t1391: F, t9132: F, t26992: F, t8392: F, t27337: F, t26854: F, t104578: F, t11593: F, t12334: F, t12754: F, t12968: F, t13135: F, t13153: F, t1359: F, t144: F, t1901: F, t2224: F, t23563: F, t23567: F, t23571: F, t23582: F, t26995: F, t446: F, t49634: F, t574: F, t605: F, t9419: F, t95749: F, t95762: F, t95771: F) -> (F,) {
    let t106875 = t2101 * t6685;
    let t106894 = t9132 * t1391;
    let t106906 = 2.0 / 27.0 * t8392 * t26992;
    let t106912 = 4.0 / 3.0 * t8392 * t27337;
    let t106914 = 2.0 / 27.0 * t8392 * t26854;
    let t106915 = -2.0 / 27.0 * t95749 + 2.0 / 9.0 * t1901 * t106875 * t2224 + 4.0 / 9.0 * t11593 * t9419 * t26995 + 2.0 / 9.0 * t1901 * t13153 * t23582 + t1901 * t13153 * t23563 / 9.0 + 2.0 / 27.0 * t1901 * t49634 * t23567 + 2.0 / 3.0 * t446 * t144 * t104578 - 4.0 / 9.0 * t1901 * t106894 * t12334 + t446 * t574 * t605 * t1359 * t13135 / 3.0 - t95762 / 27.0 - 2.0 / 81.0 * t95771 - t106906 - 2.0 / 3.0 * t1901 * t12968 * t23571 * t12754 + t106912 - t106914;
    (t106915,)
}
