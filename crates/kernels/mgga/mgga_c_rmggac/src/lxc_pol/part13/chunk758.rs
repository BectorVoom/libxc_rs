//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 758/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk758<F: Float>(t7244: F, t9159: F, t1971: F, t3351: F, t5156: F, t7190: F, t1607: F, t1986: F, t7720: F, t7279: F, t8365: F, t1979: F, t1982: F, t201: F, t597: F, t998: F) -> (F, F, F, F, F) {
    let t38934 = t7244 * t9159;
    let t38938 = t3351 * t1971 * t7190 * t5156;
    let t38943 = t1986 * t1607;
    let t38944 = t7720 * t38943;
    let t38946 = t8365 * t7279;
    let t38958 = t998 * t597 * t201 * t1979 * t1982;
    (t38934, t38938, t38944, t38946, t38958)
}
