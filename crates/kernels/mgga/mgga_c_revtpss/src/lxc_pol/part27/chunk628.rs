//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 628/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk628<F: Float>(t1412: F, t73: F, t3829: F, t1394: F, t3889: F, t1392: F, t1395: F, t4045: F, t539: F, t541: F) -> (F, F, F) {
    let t4049 = t73 * t1412;
    let t4050 = t4049 * t3829;
    let t4053 = t1394 * t3889;
    let t4056 = 6.0 * t1392 * t1395 - t4045 * t541 - 12.0 * t4050 * t539 + 3.0 * t4053 * t539;
    (t4050, t4053, t4056)
}
