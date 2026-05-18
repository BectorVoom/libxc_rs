//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 772/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk772<F: Float>(t2079: F, t262: F, t265: F, t866: F, t833: F, t2068: F, t321: F, t830: F, t2067: F, t25529: F, t839: F, t7829: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t35869 = t2079 * t262 * t265 * t866;
    let t35871 = t265 * t833;
    let t35872 = t262 * t35871;
    let t35873 = t2068 * t35872;
    let t35875 = t830 * t321;
    let t35876 = t262 * t35875;
    let t35877 = t2068 * t35876;
    let t35879 = t25529 * t2067;
    let t35884 = t265 * t839;
    let t35885 = t262 * t35884;
    let t35886 = t7829 * t35885;
    (t35869, t35871, t35872, t35873, t35875, t35876, t35877, t35879, t35884, t35885, t35886)
}
