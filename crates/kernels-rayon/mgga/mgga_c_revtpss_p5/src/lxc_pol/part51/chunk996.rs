//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 996/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk996(t247: f64, t3116: f64, t33768: f64, t1646: f64, t1984: f64, t359: f64, t1035: f64, t1668: f64, t8507: f64, t31892: f64, t1089: f64, t1651: f64, t1695: f64, t31897: f64, t31913: f64, t31920: f64, t31928: f64, t31940: f64, t31943: f64, t31972: f64, t31975: f64, t31981: f64, t31986: f64, t32014: f64, t33751: f64, t33756: f64, t33761: f64, t33765: f64, t7837: f64, t8502: f64, t8508: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33770 = t247 * t3116 * t33768;
    let t33774 = t1984 * t359 * t1646;
    let t33787 = t1035 * t1668;
    let t33791 = t8507 * t1646;
    let t33792 = t31892 * t33791;
    let t33795 = 0.56468933516960933998e-3_f64 * t31913 * t33751 - 0.56468933516960933998e-3_f64 * t31920 * t33756 + 0.28234466758480466999e-3_f64 * t31975 * t33761 + t31972 + 0.18822977838986977999e-3_f64 * t32014 * t33765 - 0.28234466758480466999e-3_f64 * t8502 * t33770 - 0.17347256376410398924e1_f64 * t31986 * t33774 + 0.17347256376410398924e1_f64 * t31943 * t1984 * t359 * t1651 - 0.17347256376410398924e1_f64 * t31940 * t7837 + 0.17347256376410398924e1_f64 * t8508 * t31981 * t359 * t1695 - 0.8673628188205199462e0_f64 * t31928 * t33787 * t1089 + 0.17135921299530705785e1_f64 * t31897 * t33792;
    (t33770, t33774, t33787, t33791, t33792, t33795)
}
