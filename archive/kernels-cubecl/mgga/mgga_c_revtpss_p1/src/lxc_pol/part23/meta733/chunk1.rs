//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2505/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2505<F: Float>(t10722: F, t4345: F, t40710: F, t4349: F, t14834: F, t9775: F, t10716: F, t14857: F, t124: F, t4423: F, t1558: F, t231: F, t40406: F, t685: F, t72: F, t826: F) -> (F, F, F, F, F, F) {
    let t50383 = t10722 * t4345;
    let t50385 = t40710 * t4349;
    let t50387 = t9775 * t14834;
    let t50389 = t10716 * t14857;
    let t50390 = F::cast_from(0.16262400898971305032e-2_f64) * t50389;
    let t50412 = t124 * t4423;
    let t50436 = t40406 * t826 * t1558 * t231 * t72 * t685;
    (t50383, t50385, t50387, t50390, t50412, t50436)
}
