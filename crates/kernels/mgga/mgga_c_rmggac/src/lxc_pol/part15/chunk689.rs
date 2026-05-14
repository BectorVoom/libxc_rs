//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 689/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk689<F: Float>(t1965: F, t35511: F, t1977: F, t1982: F, t265: F, t4789: F, t638: F, t71: F, t7311: F, t321: F, t7817: F, t1550: F, t333: F, t903: F, t338: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t35512 = t1965 * t35511;
    let t35514 = t1977 * t35512 * t1982;
    let t35565 = t638 * t265 * t4789 * t71 * t7311;
    let t35566 = 0.24390119833260022651e-2 * t35565;
    let t35583 = t7817 * t321;
    let t35584 = t1550 * t35583;
    let t35586 = t7817 * t333;
    let t35587 = t903 * t35586;
    let t35589 = t338 * t830;
    (t35512, t35514, t35566, t35583, t35584, t35586, t35587, t35589)
}
