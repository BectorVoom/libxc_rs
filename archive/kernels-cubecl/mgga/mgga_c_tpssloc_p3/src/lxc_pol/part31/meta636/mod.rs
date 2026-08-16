//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1901;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1902;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1903;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta636<F: Float>(t1985: F, t20009: F, t214: F, t225: F, t567: F, t3886: F, t6439: F, t1307: F, t22633: F, t22635: F, t26193: F, t26202: F, t6888: F, t6891: F, t97511: F, t28116: F, t80650: F, t1808: F, t254: F, t1377: F, t6347: F, t1385: F, t1842: F, t90516: F, t1992: F, t26355: F, t90566: F, t26331: F, t20022: F, t6889: F, t6906: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97604, t97611, t97616) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1901::<F>(t1985, t20009, t214, t225, t567, t3886, t6439, t1307, t22633, t22635, t26193, t26202);
        let (t97619, t97624, t97626, t97640) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1902::<F>(t6888, t6891, t97511, t22633, t28116, t80650, t1808, t254, t1377, t6347, t1385, t22635);
        let (t97644, t97647, t97652, t97658) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1903::<F>(t1842, t22633, t22635, t90516, t1992, t26355, t90566, t1307, t26331, t567, t6347, t1985, t20022, t6889, t6906);
    (t97604, t97611, t97616, t97619, t97624, t97626, t97640, t97644, t97647, t97652, t97658)
}
