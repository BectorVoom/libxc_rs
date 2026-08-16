//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1574/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1574<F: Float>(t22789: F, t72: F, t757: F, t1317: F, t22790: F, t1320: F, t512: F, t749: F, t221: F, t22954: F, t4018: F, t4019: F) -> (F, F, F, F, F) {
    let t85912 = t22789 * t72 * t757;
    let t85929 = t1317 * t22790;
    let t85931 = t1320 * t22790;
    let t85986 = t512 * t22789 * t749;
    let t86061 = t4018 * t4019 * t221 * t22954;
    (t85912, t85929, t85931, t85986, t86061)
}
