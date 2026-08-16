//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1712;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta414<F: Float>(t1385: F, t5353: F, t3887: F, t16413: F, t539: F, t225: F, t5217: F, t1834: F, t3752: F, t1323: F, t5318: F, t16122: F, t562: F, t1842: F, t3911: F, t3888: F, t12021: F, t12033: F, t1375: F, t1386: F, t1843: F, t3758: F, t3882: F, t3889: F, t5215: F, t5326: F, t5354: F, t568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16452, t16453, t16458, t16460, t16463, t16465, t16468) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1712::<F>(t1385, t5353, t3887, t16413, t539, t225, t5217, t1834, t3752, t1323, t5318, t16122, t562);
        let (t16470, t16471, t16474, t16475, t16485) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1713::<F>(t1842, t3911, t3887, t3888, t12021, t12033, t1375, t1386, t16453, t16458, t16460, t16463, t16465, t16468, t1843, t3758, t3882, t3889, t5215, t5326, t5354, t568);
    (t16452, t16453, t16458, t16460, t16463, t16465, t16468, t16470, t16471, t16474, t16475, t16485)
}
