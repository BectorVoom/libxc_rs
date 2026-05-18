//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1166/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1166<F: Float>(t2042: F, t29480: F, t2170: F, t28268: F, t28277: F, t28265: F, t28280: F, t35018: F, t575: F, t125211: F, t125213: F, t125215: F, t125217: F, t129251: F, t129253: F, t129255: F, t129257: F, t129273: F, t129277: F, t129279: F, t1518: F, t2322: F, t33346: F, t33550: F, t33578: F, t33580: F, t33583: F, t34882: F, t4254: F, t4297: F, t651: F) -> (F, F, F, F, F, F, F) {
    let t129564 = t29480 * t2042;
    let t129570 = t2170 * t28268;
    let t129572 = t2170 * t28277;
    let t129574 = t2170 * t28265;
    let t129577 = t2170 * t28280;
    let t131183 = t35018 * t575;
    let t131200 = -F::new(2.0) * t1518 * t33550 * t651 - F::new(2.0) * t2322 * t34882 - F::new(2.0) * t33346 * t4297 - F::new(2.0) * t34882 * t4254 - t125211 - t125213 - t125215 - t125217 - F::new(4.0) * t129251 - F::new(4.0) * t129253 - F::new(4.0) * t129255 - F::new(4.0) * t129257 - F::new(4.0) * t129273 - F::new(4.0) * t129277 - F::new(4.0) * t129279 - t33578 - t33580 - t33583;
    (t129564, t129570, t129572, t129574, t129577, t131183, t131200)
}
