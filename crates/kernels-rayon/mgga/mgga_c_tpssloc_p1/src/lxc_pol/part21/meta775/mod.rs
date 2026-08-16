//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta775 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2683;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta775(t39510: f64, t39512: f64, t39514: f64, t39522: f64, t39530: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39518: f64, t39521: f64, t39529: f64, t39532: f64, t19572: f64, t67: f64, t758: f64, t39540: f64, t54428: f64, t16018: f64, t16490: f64, t193: f64, t19924: f64, t20093: f64, t3918: f64, t3919: f64, t39539: f64, t39549: f64, t39563: f64, t5122: f64, t5126: f64, t55224: f64, t6347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56365, t56366, t56367, t56368, t56369, t56370) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2683(t39510, t39512, t39514, t39522, t39530, t39499, t39502, t39505, t39508, t39518, t39521, t39529);
        let (t56372, t56375, t56381, t56388, t56389) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2684(t39532, t19572, t67, t758, t39540, t54428, t16018, t16490, t193, t19924, t20093, t3918, t3919, t39539, t39549, t39563, t5122, t5126, t55224, t6347);
    (t56365, t56366, t56367, t56368, t56369, t56370, t56372, t56375, t56381, t56388, t56389)
}
