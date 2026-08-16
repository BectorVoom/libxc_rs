//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 950/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk950<F: Float>(t2007: F, t7002: F, t651: F, t2322: F, t8461: F, t4254: F, t1310: F, t8460: F, t4147: F, t7311: F, t2034: F, t2014: F) -> (F, F, F, F, F, F, F, F) {
    let t32103 = t2007 * t7002;
    let t32104 = t651 * t32103;
    let t32106 = t2322 * t8461;
    let t32107 = F::cast_from(2.0_f64) * t32106;
    let t32108 = t4254 * t8461;
    let t32109 = F::cast_from(2.0_f64) * t32108;
    let t32110 = t1310 * t8460;
    let t32111 = t651 * t32110;
    let t32112 = F::cast_from(2.0_f64) * t32111;
    let t32113 = t4147 * t7311;
    let t32114 = t2034 * t32113;
    let t32116 = F::cast_from(2.0_f64) * t2014 * t32114;
    (t32103, t32104, t32107, t32109, t32110, t32112, t32114, t32116)
}
