//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 720/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk720<F: Float>(t44415: F, t42673: F, t1063: F, t35908: F, t894: F, t13304: F, t2312: F, t13307: F, t6305: F, t13262: F, t6313: F, t13327: F, t13277: F, t11271: F, t2268: F, t2349: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44416 = 0.11856252764865062333e-2 * t44415;
    let t44420 = 0.63233348079280332443e-2 * t42673;
    let t44423 = 0.28455006635676149599e-1 * t1063 * t894 * t35908;
    let t44424 = t2312 * t13304;
    let t44425 = 0.11856252764865062333e-2 * t44424;
    let t44435 = 0.19918504644973304719e0 * t6305 * t13307;
    let t44437 = 0.7588001769513639893e-1 * t6313 * t13262;
    let t44439 = 0.37940008847568199465e-1 * t6313 * t13327;
    let t44443 = 0.22764005308540919679e0 * t6313 * t13277;
    let t44457 = 0.85365019907028448797e-1 * t2268 * t11271 * t2349;
    (t44416, t44420, t44423, t44425, t44435, t44437, t44439, t44443, t44457)
}
