//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1381/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1381<F: Float>(t27232: F, t8392: F, t26936: F, t2157: F, t26590: F, t105368: F, t105493: F, t105534: F, t105550: F, t1060: F, t11593: F, t12703: F, t12950: F, t12968: F, t13147: F, t13212: F, t144: F, t1901: F, t2190: F, t23443: F, t23571: F, t23884: F, t26924: F, t27096: F, t27333: F, t27336: F, t446: F, t50229: F, t50268: F, t574: F, t597: F, t63755: F, t6639: F, t95958: F) -> (F, F) {
    let t107193 = 4.0 / 27.0 * t8392 * t27232;
    let t107210 = 2.0 / 27.0 * t8392 * t26936;
    let t107227 = t26590 * t2157;
    let t107231 = -4.0 * t1901 * t27333 * t597 * t27336 + 2.0 / 9.0 * t1901 * t23443 * t13147 + t107193 + 8.0 / 27.0 * t11593 * t13212 * t105493 - 4.0 / 9.0 * t1901 * t50229 * t27096 - 4.0 / 9.0 * t1901 * t12703 * t105534 - 4.0 / 9.0 * t1901 * t12703 * t105550 - 2.0 / 9.0 * t1901 * t12703 * t105368 + t107210 + 8.0 / 3.0 * t1901 * t63755 * t6639 * t2190 - 4.0 / 3.0 * t1901 * t50268 * t26924 - t446 * t574 * t1060 * t23884 / 3.0 - 2.0 / 9.0 * t95958 - 2.0 / 3.0 * t1901 * t12968 * t23571 * t12950 - t446 * t144 * t107227 / 3.0;
    (t107227, t107231)
}
