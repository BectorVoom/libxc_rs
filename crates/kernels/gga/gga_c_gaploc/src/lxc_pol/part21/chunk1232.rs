//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1232/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1232<F: Float>(t12078: F, t1397: F, t1424: F, t31068: F, t34879: F, t34881: F, t34889: F, t34893: F, t34894: F, t34897: F, t34900: F, t34903: F, t34905: F, t34910: F, t34912: F, t34914: F, t34917: F, t34919: F) -> (F,) {
    let t38770 = t1397 * t12078;
    let t38773 = t34879 + t34881 + t34889 - t34893 + t34894 - t31068 + t34897 + t34900 + t34903 - t34905 - 0.79445533226334281486e-1 * t38770 * t1424 - t34910 + t34912 + t34914 + t34917 + t34919;
    (t38773,)
}
