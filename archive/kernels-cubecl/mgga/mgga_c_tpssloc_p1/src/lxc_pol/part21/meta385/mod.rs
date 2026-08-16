//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1847;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta385<F: Float>(t13546: F, t977: F, t13555: F, t2979: F, t13528: F, t13532: F, t10214: F, t13537: F, t13969: F, t4595: F, t3130: F, t1616: F, t2780: F, t3071: F, t2771: F, t10408: F, t1539: F, t3121: F, t3048: F, t4571: F, t10390: F, t10891: F, t10904: F, t10937: F, t10957: F, t1622: F, t3070: F, t3098: F, t4575: F, t4596: F, t4600: F, t4644: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14006, t14009, t14012, t14015, t14018, t14025, t14027, t14032) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1847::<F>(t13546, t977, t13555, t2979, t13528, t13532, t10214, t13537, t13969, t4595, t3130, t1616, t2780);
        let (t14033, t14036, t14037, t14040, t14041, t14049, t14050) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1848::<F>(t14032, t3071, t1616, t2771, t10408, t1539, t3121, t3048, t4571, t10390, t10891, t10904, t10937, t10957, t14006, t14009, t14012, t14015, t14018, t14027, t1622, t3070, t3098, t4575, t4596, t4600, t4644, t973);
    (t14025, t14027, t14032, t14033, t14036, t14037, t14040, t14041, t14049, t14050)
}
