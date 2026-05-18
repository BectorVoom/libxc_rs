//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 703/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk703<F: Float>(t10029: F, t1971: F, t1970: F, t8443: F, t8451: F, t530: F, t8876: F, t1945: F, t1986: F, t675: F, t1859: F, t194: F) -> (F, F, F, F, F, F, F) {
    let t10030 = t1971 * t10029;
    let t10031 = t1970 * t10030;
    let t10032 = F::new(0.85129199786595678796e-5) * t10031;
    let t10033 = t8451 * t8443;
    let t10034 = F::new(0.85129199786595678796e-5) * t10033;
    let t10036 = t530 * t8876;
    let t10037 = F::new(0.4726e1) * t10036;
    let t10040 = t1986 * t1945;
    let t10041 = t675 * t10040;
    let t10042 = F::new(0.85129199786595678796e-5) * t10041;
    let t10043 = t194 * t1859;
    (t10030, t10032, t10034, t10037, t10040, t10042, t10043)
}
