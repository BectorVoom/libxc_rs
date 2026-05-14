//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1404/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1404<F: Float>(t48691: F, t9486: F, t1520: F, t33702: F, t4170: F, t2331: F, t32440: F, t4497: F, t6204: F, t33961: F, t9532: F, t32388: F, t9860: F, t20160: F, t33831: F, t9536: F) -> (F, F, F, F, F, F) {
    let t114972 = 4.0 * t48691 * t9486;
    let t114975 = 4.0 * t4170 * t33702 * t1520;
    let t114978 = t6204 * t32440 * t2331 * t4497;
    let t114982 = 0.34722222222222222222e-2 * t33961 * t9532;
    let t114983 = t9860 * t32388;
    let t114991 = t9536 * t20160 * t33831;
    (t114972, t114975, t114978, t114982, t114983, t114991)
}
