//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 563/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk563(t14589: f64, t2147: f64, t13982: f64, t13986: f64, t13990: f64, t13994: f64, t14005: f64, t14013: f64, t14054: f64, t14060: f64, t14094: f64, t22: f64, t2227: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14590 = t14589 * t2147;
    let t14591 = 0.68186654135613354322e-2_f64 * t14590;
    let t14592 = 0.30487649791575028312e-3_f64 * t13982;
    let t14593 = 0.30487649791575028312e-3_f64 * t13986;
    let t14594 = 0.20455996240684006298e-1_f64 * t13990;
    let t14595 = 0.2727466165424534173e-1_f64 * t13994;
    let t14596 = 0.13637330827122670865e-1_f64 * t14005;
    let t14598 = 0.2627895913935205078e-5_f64 * t14013;
    let t14607 = 0.19709219354514038085e-5_f64 * t14054;
    let t14609 = 0.2627895913935205078e-5_f64 * t14060;
    let t14616 = 0.10227998120342003148e-1_f64 * t14094;
    let t14617 = t2227 * t22;
    (t14591, t14592, t14593, t14594, t14595, t14596, t14598, t14607, t14609, t14616, t14617)
}
