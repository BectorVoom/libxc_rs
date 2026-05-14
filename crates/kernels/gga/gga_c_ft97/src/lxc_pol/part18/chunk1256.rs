//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1256/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1256<F: Float>(t26476: F, t376: F, t89: F, t1882: F, t26207: F, t370: F, t8418: F, t26392: F, t26280: F, t100074: F, t100335: F, t102268: F, t102350: F, t102401: F, t110: F, t11867: F, t1871: F, t1876: F, t1901: F, t23129: F, t446: F, t447: F, t47659: F, t6547: F, t83: F, t91539: F, t92053: F, t92059: F, t92062: F, t925: F) -> (F,) {
    let t103607 = 2.0 / 9.0 * t89 * t376 * t26476;
    let t103625 = 2.0 / 27.0 * t1882 * t26207;
    let t103626 = t370 * t8418;
    let t103632 = 2.0 / 9.0 * t1882 * t26392;
    let t103640 = 2.0 / 9.0 * t1882 * t26280;
    let t103641 = -t446 * t447 * t23129 * t925 / 9.0 - t103607 - 2.0 / 3.0 * t446 * t83 * t102401 + 4.0 / 3.0 * t446 * t83 * t100074 + 2.0 / 3.0 * t446 * t1871 * t110 * t100335 - 2.0 / 27.0 * t92053 - 2.0 / 27.0 * t92059 + 8.0 / 81.0 * t92062 + 4.0 / 9.0 * t47659 * t91539 * t11867 + t103625 + 4.0 * t1901 * t103626 * t6547 * t1876 - t103632 - 2.0 / 3.0 * t446 * t83 * t102350 - t446 * t83 * t102268 / 3.0 + t103640;
    (t103641,)
}
