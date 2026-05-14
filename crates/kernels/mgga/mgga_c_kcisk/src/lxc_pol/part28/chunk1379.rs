//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1379/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1379<F: Float>(t23276: F, t2789: F, t415: F, t1333: F, t35205: F, t17353: F, t35122: F, t33056: F, t33031: F, t35149: F, t116664: F, t2528: F, t1869: F, t34089: F, t34107: F, t34045: F, t34125: F) -> (F, F, F, F, F, F, F, F) {
    let t121765 = t415 * t23276 * t2789;
    let t121767 = t1333 * t35205;
    let t121769 = t17353 * t35122;
    let t121770 = t33056 * t121769;
    let t121772 = t33031 * t121769;
    let t121774 = t1333 * t35149;
    let t121777 = t415 * t116664 * t2528;
    let t121787 = t1869 * t34107 * t34089;
    let t121789 = t34125 * t34045;
    (t121765, t121767, t121770, t121772, t121774, t121777, t121787, t121789)
}
