//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2506/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2506<F: Float>(t1208: F, t12689: F, t225: F, t480: F, t3671: F, t3672: F, t371: F, t676: F, t12625: F, t458: F, t456: F, t43813: F) -> (F, F, F, F, F, F, F) {
    let t44831 = t12689 * t1208;
    let t44832 = t44831 * t225;
    let t44833 = t44832 * t480;
    let t44838 = t3671 * t371 * t676 * t3672;
    let t44841 = F::new(1.0) / t12625 / t458;
    let t44842 = t456 * t44841;
    let t44843 = t44842 * t225;
    let t44865 = F::cast_from(0.15365432098765432099e0_f64) * t43813;
    (t44831, t44832, t44833, t44838, t44842, t44843, t44865)
}
