//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1844/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1844<F: Float>(t25304: F, t7283: F, t25946: F, t25949: F, t786: F, t7286: F, t225: F, t26034: F, t1426: F, t3999: F, t26044: F, t4003: F) -> (F, F, F, F, F, F, F) {
    let t26069 = t25304 * t7283;
    let t26071 = F::cast_from(0.22849835011101738147e-2_f64) * t26069 * t25946;
    let t26072 = t786 * t25949;
    let t26073 = t26072 * t7286;
    let t26075 = t26034 * t225;
    let t26079 = t1426 * t3999;
    let t26080 = t26044 * t4003;
    (t26069, t26071, t26072, t26073, t26075, t26079, t26080)
}
