//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1175/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1175<F: Float>(t38953: F, t7102: F, t29142: F, t8392: F, t29138: F, t29256: F, t1882: F, t29363: F, t10696: F, t1476: F, t29084: F, t29389: F, t7111: F, t8232: F, t1501: F, t44600: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t113800 = t38953 * t7102;
    let t113807 = 4.0 / 9.0 * t8392 * t29142;
    let t113809 = 4.0 / 9.0 * t8392 * t29138;
    let t113816 = 2.0 / 27.0 * t8392 * t29256;
    let t113831 = 2.0 / 9.0 * t1882 * t29363;
    let t113847 = t10696 * t1476;
    let t113856 = 2.0 / 27.0 * t8392 * t29084;
    let t113866 = 2.0 / 9.0 * t1882 * t29389;
    let t113867 = t8232 * t7111;
    let t113869 = t44600 * t1501;
    (t113800, t113807, t113809, t113816, t113831, t113847, t113856, t113866, t113867, t113869)
}
