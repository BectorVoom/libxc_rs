//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1163/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1163<F: Float>(t1651: F, t7817: F, t7145: F, t25672: F, t3304: F, t6305: F, t3318: F, t7168: F, t1695: F, t7160: F, t1976: F, t6244: F) -> (F, F, F, F, F, F) {
    let t29843 = t7817 * t1651;
    let t29844 = t7145 * t29843;
    let t29848 = t25672 * t6305 * t3304;
    let t29852 = t7168 * t6305 * t3318;
    let t29865 = t7817 * t1695;
    let t29866 = t7160 * t29865;
    let t29871 = t1976 * t6244;
    (t29843, t29844, t29848, t29852, t29866, t29871)
}
