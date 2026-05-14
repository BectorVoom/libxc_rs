//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 920/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk920<F: Float>(t2180: F, t23997: F, t144: F, t1386: F, t8232: F, t1882: F, t5953: F, t376: F, t5931: F, t89: F, t2212: F, t5916: F, t9144: F, t23602: F, t23606: F, t23613: F, t23616: F, t23619: F, t23623: F, t23627: F, t23629: F, t23634: F, t23639: F, t23643: F, t23647: F, t23650: F, t23655: F, t23661: F, t23664: F) -> (F, F, F, F, F, F, F, F) {
    let t23998 = t23997 * t2180;
    let t23999 = t144 * t23998;
    let t24003 = 4.0 / 27.0 * t8232 * t1386;
    let t24004 = t1882 * t5953;
    let t24007 = t89 * t376 * t5931;
    let t24009 = t5916 * t2212;
    let t24010 = t9144 * t24009;
    let t24028 = -2.0 * t23602 - t23606 - t23613 / 8.0 - t23616 / 18.0 - 4.0 / 9.0 * t23619 + 2.0 / 3.0 * t23623 + t23627 / 3.0 - 2.0 / 9.0 * t23629 + t23634 / 6.0 + t23639 / 12.0 + t23643 / 18.0 + t23647 / 27.0 - t23650 / 27.0 + t23655 / 9.0 - t23661 / 3.0 - t23664 / 9.0;
    (t23998, t23999, t24003, t24004, t24007, t24009, t24010, t24028)
}
