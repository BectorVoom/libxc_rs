//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1153/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1153<F: Float>(t32276: F, t32307: F, t1459: F, t32247: F, t32249: F, t32251: F, t32253: F, t32256: F, t32258: F, t32262: F, t32264: F, t32267: F, t32270: F, t32272: F, t32274: F, t32279: F, t32281: F, t32283: F, t32285: F, t32288: F, t32291: F, t32293: F, t32296: F, t32299: F, t32301: F, t32303: F, t32305: F) -> (F, F, F, F) {
    let t32308 = t32276 + t32307;
    let t32309 = t1459 * t32308;
    let t32322 = 0.28777777777777777778e0 * t32247 - 0.68347222222222222224e0 * t32249 - 0.89930555555555555557e-2 * t32251 + 0.20234375e-1 * t32253 + 0.5e0 * t32256 - 0.125e0 * t32258 + 0.1875e0 * t32262 - 0.5e0 * t32264 + 0.125e0 * t32267 - 0.1875e0 * t32270 - 0.4046875e-1 * t32272 - 0.21583333333333333334e0 * t32274;
    let t32335 = 0.53958333333333333334e-1 * t32279 + 0.9375e-1 * t32281 + 0.4046875e-1 * t32283 + 0.91666666666666666667e0 * t32285 - 0.33333333333333333334e0 * t32288 + 0.625e-1 * t32291 - 0.20234375e-1 * t32293 - 0.9375e-1 * t32296 - 0.20833333333333333333e-1 * t32299 + 0.21583333333333333334e0 * t32301 - 0.53958333333333333334e-1 * t32303 - 0.26979166666666666667e-1 * t32305;
    (t32308, t32309, t32322, t32335)
}
