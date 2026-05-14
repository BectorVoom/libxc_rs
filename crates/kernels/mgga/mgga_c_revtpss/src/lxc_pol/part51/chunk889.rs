//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 889/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk889<F: Float>(t1651: F, t385: F, t247: F, t3116: F, t1045: F, t1668: F, t3117: F, t1592: F, t32016: F, t32015: F, t1078: F, t1695: F, t1646: F, t1984: F, t359: F, t1035: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33754 = t385 * t1651;
    let t33756 = t247 * t3116 * t33754;
    let t33760 = t385 * t1668 * t1045;
    let t33761 = t3117 * t33760;
    let t33764 = t32016 * t1592;
    let t33765 = t32015 * t33764;
    let t33768 = t1078 * t1695;
    let t33770 = t247 * t3116 * t33768;
    let t33774 = t1984 * t359 * t1646;
    let t33787 = t1035 * t1668;
    (t33754, t33756, t33760, t33761, t33764, t33765, t33768, t33770, t33774, t33787)
}
