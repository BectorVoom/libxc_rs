//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1347/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1347<F: Float>(t23919: F, t23921: F, t19425: F, t19427: F, t19429: F, t41: F, t457: F, t9904: F, t32195: F, t88: F, t19476: F, t19478: F, t28063: F, t19421: F, t19424: F, t19611: F, t19614: F, t19620: F, t19720: F, t23918: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32976 = 360.0 * t23919;
    let t32977 = 0.30762056574649219972e4 * t23921;
    let t32978 = 0.10254018858216406658e4 * t19425;
    let t32979 = 0.35089341735807877242e1 * t19427;
    let t32980 = 60.0 * t19429;
    let t32982 = t41 * t9904 * t457;
    let t32984 = t41 * t32195 * t88;
    let t32985 = 120.0 * t19476;
    let t32986 = 24.0 * t19478;
    let t32987 = 0.51947577317044391276e2 * t28063;
    let t32988 = -t23918 - t32976 - t19421 - t32977 - t19424 - t32978 - t32979 + t32980 + t32982 + t32984 - t32985 - t32986 - t32987 + t19720 + t19611 + t19614 - t19620;
    (t32976, t32977, t32978, t32979, t32980, t32982, t32984, t32985, t32986, t32987, t32988)
}
