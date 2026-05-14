//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1098/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1098<F: Float>(t2347: F, t4565: F, t1610: F, t6602: F, t20942: F, t20944: F, t20947: F, t20949: F, t20951: F, t20953: F, t20955: F, t20959: F, t20962: F, t20965: F, t20967: F, t20970: F, t20973: F, t20976: F, t20978: F, t20981: F, t20984: F, t20986: F, t20988: F, t20991: F) -> (F, F, F) {
    let t22052 = t2347 * t4565;
    let t22056 = t6602 * t1610;
    let t22079 = 0.41666666666666666666e-1 * t20942 - 0.13489583333333333333e-1 * t20944 + 0.61111111111111111112e0 * t20947 - 0.14388888888888888889e0 * t20949 + 0.10791666666666666667e0 * t20951 + 0.14388888888888888889e0 * t20953 + 0.125e0 * t20955 + 0.25e0 * t20959 - 0.20833333333333333333e-1 * t20962 + 0.27777777777777777777e-1 * t20965 - 0.9375e-1 * t20967 - 0.89930555555555555554e-2 * t20970 - 0.44965277777777777777e-2 * t20973 - 0.91666666666666666667e0 * t20976 + 0.101171875e-1 * t20978 + 0.34173611111111111111e0 * t20981 - 0.10791666666666666667e0 * t20984 - 0.10791666666666666667e0 * t20986 - 0.44965277777777777777e-2 * t20988 - 0.4046875e-1 * t20991;
    (t22052, t22056, t22079)
}
