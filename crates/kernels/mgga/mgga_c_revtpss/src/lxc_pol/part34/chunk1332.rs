//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1332/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1332<F: Float>(t114752: F, t2035: F, t29499: F, t7898: F, t29495: F, t29506: F, t7937: F, t2014: F, t2034: F, t86791: F, t114363: F, t114434: F, t114436: F, t114438: F, t114440: F, t114442: F, t114445: F, t114451: F, t114455: F, t114746: F, t1843: F, t2011: F, t22578: F, t23094: F, t29573: F, t508: F, t6765: F, t6934: F, t6985: F, t7725: F, t7894: F) -> F {
    let t114753 = t114752 * t2035;
    let t114755 = F::new(18.0) * t7898 * t29499;
    let t114757 = F::new(9.0) * t7898 * t29495;
    let t114759 = F::new(3.0) * t29506 * t7937;
    let t114765 = F::new(6.0) * t2014 * t2034 * t86791;
    let t114766 = -F::new(6.0) * t114363 * t508 - F::new(6.0) * t1843 * t29573 + t2011 * t23094 - F::new(6.0) * t22578 * t6985 - F::new(3.0) * t6765 * t7725 + F::new(3.0) * t6934 * t7894 - t114434 - t114436 - t114438 - t114440 - t114442 + t114445 + t114451 - t114455 + t114746 + t114753 + t114755 + t114757 - t114759 - t114765;
    t114766
}
