//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1447/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1447<F: Float>(t122193: F, t122195: F, t122197: F, t122199: F, t122201: F, t122203: F, t122205: F, t122207: F, t122209: F, t122211: F, t122213: F, t122239: F, t122241: F, t122243: F, t122245: F, t122248: F, t122250: F, t122252: F, t122254: F, t122256: F, t122258: F, t122260: F) -> (F, F) {
    let t123365 = -0.625e-1 * t122193 + 0.91666666666666666667e0 * t122195 - 0.26979166666666666667e-1 * t122197 - 1.0 * t122199 - 0.91129629629629629632e0 * t122201 - 0.125e0 * t122203 - 0.125e0 * t122205 - 0.125e0 * t122207 + 0.53958333333333333333e-1 * t122209 + 0.125e0 * t122211 + 0.68347222222222222224e0 * t122213;
    let t123389 = 0.375e0 * t122239 - 0.33333333333333333334e0 * t122241 + 0.20234375e-1 * t122243 + 0.5e0 * t122245 - 0.33333333333333333333e0 * t122248 + 0.20234375e-1 * t122250 - 0.21583333333333333334e0 * t122252 + 0.28777777777777777779e0 * t122254 + 0.20833333333333333333e-1 * t122256 - 0.41666666666666666667e-1 * t122258 - 0.125e0 * t122260;
    (t123365, t123389)
}
