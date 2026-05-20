//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2233/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2233<F: Float>(t28043: F, t4248: F, t651: F, t6765: F, t7002: F, t108716: F, t108718: F, t108721: F, t108723: F, t108725: F, t108727: F, t109006: F, t109012: F, t109014: F, t1310: F, t2007: F, t21814: F, t21891: F, t25805: F, t28025: F, t28030: F, t28050: F, t29569: F, t4297: F, t508: F, t5877: F, t5887: F, t6985: F, t7221: F, t7732: F) -> F {
    let t109024 = F::new(4.0) * t4248 * t28043;
    let t109029 = F::new(2.0) * t651 * t6765 * t7002;
    let t109030 = -t109006 * t508 - t1310 * t29569 - t2007 * t21814 - F::new(4.0) * t21891 * t6985 - F::new(4.0) * t25805 * t5887 - F::new(4.0) * t28025 * t5887 - F::new(4.0) * t28030 * t4297 - F::new(4.0) * t28050 * t7732 - t5877 * t7221 - t108716 - t108718 - t108721 - t108723 - t108725 - t108727 - t109012 + t109014 - t109024 - t109029;
    t109030
}
