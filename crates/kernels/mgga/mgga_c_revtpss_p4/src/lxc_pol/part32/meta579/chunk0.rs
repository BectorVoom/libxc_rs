//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1906/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1906<F: Float>(t1445: F, t28824: F, t689: F, t102274: F, t25878: F, t102100: F, t26069: F, t26231: F, t98380: F, t13730: F, t2098: F, t2782: F) -> (F, F, F, F, F) {
    let t102361 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t28824 * t1445;
    let t102363 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t102274;
    let t102364 = t26069 * t102100;
    let t102367 = F::cast_from(0.25702851531048074406e-1_f64) * t98380 * t26231;
    let t102372 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t2098 * t13730;
    (t102361, t102363, t102364, t102367, t102372)
}
