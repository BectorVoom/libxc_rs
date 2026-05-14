//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 978/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk978<F: Float>(t50: F, t238: F, t34: F, t821: F, t12177: F, t1289: F, t15095: F, t1699: F, t1702: F, t2868: F, t2910: F, t35: F, t4084: F, t5468: F, t5493: F, t5498: F, t595: F, t829: F, t830: F, t833: F, zeta_threshold: F) -> (F, F) {
    let t51 = t50 <= zeta_threshold;
    let t19487 = t238 * t34 * t821;
    let t19508 = piecewise3(t51, 0.0, -56.0 / 81.0 * t12177 * t1699 * t830 - 64.0 / 27.0 * t4084 * t19487 + 8.0 / 27.0 * t5493 * t833 - 16.0 / 9.0 * t829 * t35 * t595 + 8.0 / 9.0 * t1289 * t821 - 8.0 / 3.0 * t1289 * t2868 + 8.0 / 27.0 * t2910 * t1702 * t830 - 4.0 / 9.0 * t829 * t5468 * t238 - 2.0 / 9.0 * t5498 * t833 - t15095);
    (t19487, t19508)
}
