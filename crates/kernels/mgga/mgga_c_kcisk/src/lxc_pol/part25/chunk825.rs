//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 825/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk825<F: Float>(t10501: F, t1992: F, t772: F, t1961: F, t5372: F, t10568: F, t5396: F, t760: F, t755: F, t10641: F, t1964: F, t5399: F, t763: F, t4781: F, t4790: F, t1670: F, t4761: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11983 = 0.51588271604938271604e-3 * t10501;
    let t11984 = t1992 * t1992;
    let t11985 = 1.0 / t11984;
    let t11986 = t772 * t11985;
    let t11999 = t1961 * t5372;
    let t12002 = 0.53272592592592592592e-1 * t10568;
    let t12017 = 1.0 / t5396 / t760;
    let t12018 = t755 * t12017;
    let t12042 = 0.16068111111111111111e1 * t10568;
    let t12043 = 0.46308888888888888888e0 * t10641;
    let t12058 = 1.0 / t5396 / t1964;
    let t12059 = t755 * t12058;
    let t12061 = 1.0 / t5399 / t763;
    let t12076 = t4781 * t4790;
    let t12084 = t1670 * t4761;
    (t11983, t11984, t11985, t11986, t11999, t12002, t12018, t12042, t12043, t12059, t12061, t12076, t12084)
}
