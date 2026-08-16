//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1296/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1296(t283: f64, t3225: f64, t359: f64, t14073: f64, t3200: f64, t26692: f64, t27803: f64, t44575: f64, t7703: f64, t8037: f64, t27772: f64, t2911: f64, t4781: f64) -> (f64, f64, f64, f64) {
    let t95848 = t3225 * t283 * t359;
    let t95850 = t3200 * t95848 * t14073;
    let t95852 = t26692 * t27803;
    let t95855 = t7703 * t44575 * t8037;
    let t95860 = t27772 * t4781 * t2911;
    (t95850, t95852, t95855, t95860)
}
