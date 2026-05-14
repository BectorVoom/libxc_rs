//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1042/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1042<F: Float>(t20917: F, t504: F, t1458: F, t6239: F, t1520: F, t2240: F, t4169: F, t4171: F, t4321: F, t6241: F, t14284: F, t2282: F, t14287: F, t6244: F, t4165: F, t6394: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20918 = t20917 * t504;
    let t20919 = t6239 * t1458;
    let t20921 = 2.0 * t20919 * t1520;
    let t20922 = t2240 * t4169;
    let t20924 = 2.0 * t20922 * t4171;
    let t20925 = t6241 * t4321;
    let t20926 = t14284 * t2282;
    let t20928 = 4.0 * t14287 * t6244;
    let t20930 = 2.0 * t4165 * t6394;
    (t20918, t20919, t20921, t20922, t20924, t20925, t20926, t20928, t20930)
}
