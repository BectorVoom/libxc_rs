//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1598/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1598<F: Float>(t22740: F, t3792: F, t22897: F, t1992: F, t22751: F, t6892: F, t6883: F, t6908: F, t3719: F, t6890: F, t6889: F, t6888: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22898 = t22740 * t3792;
    let t22899 = t22897 * t22898;
    let t22900 = t1992 * t22899;
    let t22907 = t22751 * t6892;
    let t22908 = F::cast_from(0.76763589786250567036e-1_f64) * t22907;
    let t22909 = t6883 * t6908;
    let t22910 = F::cast_from(0.38381794893125283518e-1_f64) * t22909;
    let t22916 = t6890 * t3719;
    let t22917 = t6889 * t22916;
    let t22918 = t6888 * t22917;
    (t22898, t22899, t22900, t22907, t22908, t22909, t22910, t22916, t22917, t22918)
}
