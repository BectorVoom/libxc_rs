//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 646/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk646<F: Float>(t10023: F, t1971: F, t1970: F, t209: F, t570: F, t605: F, t515: F, t8443: F, t8451: F, t1945: F, t1986: F, t675: F, t1859: F, t194: F, t201: F, t1979: F, t1982: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10024 = t1971 * t10023;
    let t10025 = t1970 * t10024;
    let t10028 = t570 * t605 * t209;
    let t10029 = t515 * t10028;
    let t10030 = t1971 * t10029;
    let t10031 = t1970 * t10030;
    let t10033 = t8451 * t8443;
    let t10040 = t1986 * t1945;
    let t10041 = t675 * t10040;
    let t10043 = t194 * t1859;
    let t10044 = t10043 * t201;
    let t10046 = t10044 * t1979 * t1982;
    (t10024, t10025, t10030, t10031, t10033, t10040, t10041, t10043, t10044, t10046)
}
