//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1305/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1305<F: Float>(t23421: F, t33: F, t113096: F, t25759: F, t23148: F, t1583: F, t6416: F, t23429: F, t1544: F, t113107: F, t27799: F, t113123: F, t113416: F, t1940: F, t1963: F, t2000: F, t22783: F, t2403: F, t27158: F, t27368: F, t27382: F, t29705: F, t29939: F, t29953: F, t29964: F, t29970: F, t4541: F, t7091: F, t7783: F, t7862: F, t92742: F, t98722: F) -> F {
    let t114150 = t33 * t23421;
    let t114165 = t25759 * t113096;
    let t114171 = t33 * t23148;
    let t114184 = t6416 * t1583;
    let t114188 = t33 * t23429;
    let t114192 = t6416 * t1544;
    let t114196 = t27799 * t113107;
    let t114199 = -t1940 * t7091 * t114150 / F::new(2.0) + t1940 * t113416 * t33 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t1940 * t27368 * t29970 + F::new(3.0) * t113123 * t2000 + t1940 * t1963 * t22783 / F::new(2.0) - F::new(9.0) * t27158 * t114165 + F::new(9.0) * t4541 * t7783 * t29939 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t114171 + F::new(3.0) * t1940 * t98722 * t29964 + F::new(9.0) / F::new(2.0) * t2403 * t29705 * t7862 + F::new(9.0) / F::new(2.0) * t2403 * t7783 * t29953 - F::new(3.0) / F::new(2.0) * t1940 * t7091 * t114184 - F::new(3.0) * t1940 * t92742 * t114188 + F::new(9.0) / F::new(2.0) * t2403 * t1963 * t114192 + F::new(3.0) * t27382 * t114196;
    t114199
}
