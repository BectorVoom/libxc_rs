//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2421/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2421<F: Float>(t12625: F, t458: F, t456: F, t225: F, t43813: F, t126: F, t13099: F, t1224: F, t12268: F, t1222: F, t1226: F, t2438: F) -> (F, F, F, F, F, F) {
    let t44841 = F::new(1.0) / t12625 / t458;
    let t44842 = t456 * t44841;
    let t44843 = t44842 * t225;
    let t44865 = F::cast_from(0.15365432098765432099e0_f64) * t43813;
    let t44895 = t126 * t13099;
    let t44919 = t1224 * t12268;
    let t44931 = t1222 * t2438 * t1226;
    (t44842, t44843, t44865, t44895, t44919, t44931)
}
