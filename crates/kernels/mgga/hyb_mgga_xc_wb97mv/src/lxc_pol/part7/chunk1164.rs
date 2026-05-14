//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1164/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1164<F: Float>(t546: F, t8209: F, t19: F, t2003: F, t3131: F, t1975: F, t3011: F, t8206: F, t549: F, t8164: F, t1175: F, t6478: F, t6481: F, t1183: F, t3040: F, t23604: F, t33: F, t34: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24851 = t546 * t8209;
    let t24854 = t19 * t2003 * t3131;
    let t24856 = t1975 * t3011;
    let t24858 = t546 * t8206;
    let t24861 = t19 * t549 * t8164;
    let t24876 = t1175 * t6478;
    let t24878 = t1175 * t6481;
    let t24882 = t3040 * t1183;
    let t24893 = t33 * t34 * t23604;
    (t24851, t24854, t24856, t24858, t24861, t24876, t24878, t24882, t24893)
}
