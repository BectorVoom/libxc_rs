//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1355/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1355<F: Float>(t41306: F, t367: F, t371: F, t373: F, t9291: F, t2852: F, t3154: F, t11874: F, t15688: F, t11853: F, t828: F, t3181: F, t675: F) -> (F, F, F, F, F, F) {
    let t42078 = F::cast_from(0.15365432098765432099e0_f64) * t41306;
    let t42121 = F::cast_from(0.14820648238345094262e-3_f64) * t367 * t371 * t9291 * t373;
    let t42215 = t3154 * t2852;
    let t42328 = t11874 * t15688;
    let t42410 = t828 * t11853;
    let t42447 = t675 * t3181;
    (t42078, t42121, t42215, t42328, t42410, t42447)
}
