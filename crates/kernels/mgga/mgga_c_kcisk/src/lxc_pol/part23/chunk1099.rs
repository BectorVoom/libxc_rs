//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1099/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1099<F: Float>(t21022: F, t21025: F, t21027: F, t21031: F, t21033: F, t21036: F, t21039: F, t21042: F, t21045: F, t21048: F, t21052: F, t21058: F, t21060: F, t21062: F, t21064: F, t21067: F, t21069: F, t21072: F, t21075: F, t21078: F, t21081: F, t21083: F, t21086: F, t21088: F, t21091: F, t21094: F, t21096: F, t21099: F, t21271: F, t21273: F, t21275: F) -> (F, F) {
    let t22102 = 0.33333333333333333334e0 * t21022 + 0.26979166666666666666e-1 * t21025 - 0.1875e0 * t21027 + 0.29976851851851851851e-2 * t21031 - 0.14388888888888888889e0 * t21033 + 0.47962962962962962962e-1 * t21036 - 0.53958333333333333333e-1 * t21039 - 0.45564814814814814814e0 * t21042 + 0.44965277777777777777e-2 * t21045 - 0.625e-1 * t21048 - 0.5625e0 * t21052;
    let t22125 = 0.5e0 * t21058 + 0.26979166666666666666e-1 * t21060 + 0.625e-1 * t21062 + 0.26979166666666666666e-1 * t21064 - 0.20234375e-1 * t21067 + 0.5e0 * t21069 - 0.26979166666666666666e-1 * t21072 + 0.13489583333333333333e-1 * t21075 + 0.1875e0 * t21078 + 0.11111111111111111111e0 * t21081 - 0.13489583333333333333e-1 * t21083 + 0.13489583333333333333e-1 * t21086 + 0.14388888888888888889e0 * t21088 + 0.20833333333333333333e-1 * t21091 + 0.89930555555555555554e-2 * t21094 - 0.5e0 * t21096 - 0.4046875e-1 * t21099 + 0.9375e-1 * t21271 - 0.10791666666666666667e0 * t21273 - 0.125e0 * t21275;
    (t22102, t22125)
}
