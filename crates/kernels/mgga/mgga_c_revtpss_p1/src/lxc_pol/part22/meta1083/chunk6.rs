//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3919/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3919<F: Float>(t1353: F, t5778: F, t1343: F, t1450: F, t198: F, t22279: F, t4139: F, t4140: F, t47070: F, t47072: F, t47076: F, t532: F, t5536: F, t5542: F, t73578: F, t73614: F, t73634: F, t73664: F, t73700: F, t74107: F, t74108: F, t74109: F, t74110: F, t74112: F, t74749: F, t74786: F, t74831: F, t75343: F) -> F {
    let t75353 = t1353 * t5778;
    let t75357 = F::new(3.0) * t198 * t1343 * t73578 + t198 * t532 * (t73614 + t73634 + t73664 + t73700 + t74749 + t74786 + t74831 + t75343) * t1450 - t74107 + t47070 - t47072 - t74108 - t74109 + F::new(24.0) * t5536 * t4140 * t22279 - t47076 - F::new(12.0) * t4139 * t5542 * t75353 + t74110 + t74112;
    t75357
}
