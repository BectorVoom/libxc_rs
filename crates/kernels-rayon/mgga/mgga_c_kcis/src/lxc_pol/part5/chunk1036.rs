//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1036/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1036(t14576: f64, t14607: f64, t1864: f64, t3668: f64, t12274: f64, t2003: f64, t6019: f64, t11881: f64, t1948: f64, t4142: f64, t5773: f64, t1495: f64, t4169: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15662 = 0.15476481481481481481e-2_f64 * t14576;
    let t15671 = 0.15476481481481481481e-2_f64 * t14607;
    let t15692 = t1864 * t3668;
    let t15800 = t12274 * t2003;
    let t15808 = t6019 * sigma2;
    let t15826 = t11881 * t1948;
    let t15844 = t4142 * t5773;
    let t15865 = t4169 * t1495;
    (t15662, t15671, t15692, t15800, t15808, t15826, t15844, t15865)
}
