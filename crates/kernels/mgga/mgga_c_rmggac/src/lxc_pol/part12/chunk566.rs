//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 566/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk566<F: Float>(t640: F, t7352: F, t7764: F, t2019: F, t2064: F, t333: F, t903: F, t665: F, t839: F, t1364: F, t794: F, t1550: F, t265: F, t338: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7765 = t640 * t7352;
    let t7766 = t7764 * t7765;
    let t7767 = t2019 * t7766;
    let t7769 = t2064 * t333;
    let t7770 = t903 * t7769;
    let t7772 = t665 * t839;
    let t7773 = t1364 * t7772;
    let t7774 = 0.23948483403727617128e0 * t7773;
    let t7775 = t665 * t794;
    let t7776 = t1550 * t7775;
    let t7777 = 0.11974241701863808564e0 * t7776;
    let t7778 = t338 * t265;
    (t7765, t7766, t7767, t7769, t7770, t7772, t7774, t7775, t7777, t7778)
}
