//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 890/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk890<F: Float>(t1646: F, t8507: F, t31892: F, t1089: F, t1651: F, t1695: F, t1984: F, t31897: F, t31913: F, t31920: F, t31928: F, t31940: F, t31943: F, t31972: F, t31975: F, t31981: F, t31986: F, t32014: F, t33751: F, t33756: F, t33761: F, t33765: F, t33770: F, t33774: F, t33787: F, t359: F, t7837: F, t8502: F, t8508: F) -> (F, F, F) {
    let t33791 = t8507 * t1646;
    let t33792 = t31892 * t33791;
    let t33795 = 0.56468933516960933998e-3 * t31913 * t33751 - 0.56468933516960933998e-3 * t31920 * t33756 + 0.28234466758480466999e-3 * t31975 * t33761 + t31972 + 0.18822977838986977999e-3 * t32014 * t33765 - 0.28234466758480466999e-3 * t8502 * t33770 - 0.17347256376410398924e1 * t31986 * t33774 + 0.17347256376410398924e1 * t31943 * t1984 * t359 * t1651 - 0.17347256376410398924e1 * t31940 * t7837 + 0.17347256376410398924e1 * t8508 * t31981 * t359 * t1695 - 0.8673628188205199462e0 * t31928 * t33787 * t1089 + 0.17135921299530705785e1 * t31897 * t33792;
    (t33791, t33792, t33795)
}
