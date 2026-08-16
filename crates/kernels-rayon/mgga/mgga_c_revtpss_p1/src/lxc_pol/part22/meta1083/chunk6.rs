//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3919/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3919(t1353: f64, t5778: f64, t1343: f64, t1450: f64, t198: f64, t22279: f64, t4139: f64, t4140: f64, t47070: f64, t47072: f64, t47076: f64, t532: f64, t5536: f64, t5542: f64, t73578: f64, t73614: f64, t73634: f64, t73664: f64, t73700: f64, t74107: f64, t74108: f64, t74109: f64, t74110: f64, t74112: f64, t74749: f64, t74786: f64, t74831: f64, t75343: f64) -> f64 {
    let t75353 = t1353 * t5778;
    let t75357 = 3.0_f64 * t198 * t1343 * t73578 + t198 * t532 * (t73614 + t73634 + t73664 + t73700 + t74749 + t74786 + t74831 + t75343) * t1450 - t74107 + t47070 - t47072 - t74108 - t74109 + 24.0_f64 * t5536 * t4140 * t22279 - t47076 - 12.0_f64 * t4139 * t5542 * t75353 + t74110 + t74112;
    t75357
}
