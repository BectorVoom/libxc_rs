//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2424/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2424<F: Float>(t43813: F, t1209: F, t13126: F, t17708: F, t1203: F, t12626: F, t225: F, t480: F, t12627: F, t1269: F, t44842: F, t487: F) -> (F, F, F, F, F, F, F) {
    let t45232 = F::cast_from(0.17757530864197530864e0_f64) * t43813;
    let t45371 = t1209 * t13126 * t17708;
    let t45384 = t1203 * t12626;
    let t45385 = t45384 * t225;
    let t45386 = t45385 * t480;
    let t45427 = t12627 * t1269;
    let t45438 = t44842 * t487;
    (t45232, t45371, t45384, t45385, t45386, t45427, t45438)
}
