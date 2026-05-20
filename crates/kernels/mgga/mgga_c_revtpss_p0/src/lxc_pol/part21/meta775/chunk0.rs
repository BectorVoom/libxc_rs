//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2758/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2758<F: Float>(t808: F, t853: F, t14792: F, t50768: F, t14688: F, t40731: F, t10777: F, t14671: F, t14686: F, t2754: F, t14749: F, t221: F) -> (F, F, F, F, F) {
    let t50769 = t808 * t853;
    let t50771 = t50768 * t50769 * t14792;
    let t50773 = t40731 * t14688;
    let t50774 = F::cast_from(0.40656002247428262579e-3_f64) * t50773;
    let t50784 = t10777 * t14686 * t14671 * t2754;
    let t50789 = t221 * t14749;
    (t50769, t50771, t50774, t50784, t50789)
}
