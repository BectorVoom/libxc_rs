//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2853/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2853<F: Float>(t61178: F, t61180: F, t39860: F, t18263: F, t4305: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t49958: F, t49964: F, t49982: F) -> (F, F, F, F, F) {
    let t76976 = F::new(12.0) * t61178;
    let t76977 = F::new(24.0) * t61180;
    let t76978 = F::cast_from(0.56968947174242584612e-3_f64) * t39860;
    let t76979 = t18263 * t4305;
    let t76980 = F::new(12.0) * t76979;
    let t76981 = -t49958 - t49964 - t39783 - t39786 - t39791 - t39795 + t49982 + t39799 + t76976 + t39807 - t39813 + t76977 - t39818 - t39823 - t76978 + t40084 + t76980;
    (t76976, t76977, t76978, t76980, t76981)
}
