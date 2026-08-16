//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta870 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2768;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta870<F: Float>(t13999: F, t22271: F, t48919: F, t6869: F, t9816: F, t9818: F, t13847: F, t22016: F, t48731: F, t73731: F, t1399: F, t73856: F, t22298: F, t48100: F, t22129: F, t2713: F, t3964: F, t22169: F, t46691: F, t22173: F, t9744: F, t6856: F, t9779: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t74186, t74206, t74232, t74249) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2768::<F>(t13999, t22271, t48919, t6869, t9816, t9818, t13847, t22016, t48731, t73731, t1399, t73856);
        let (t74257, t74264, t74269, t74271, t74277) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2769::<F>(t22298, t48100, t9816, t22129, t2713, t3964, t22169, t46691, t22173, t9744, t6856, t9779);
    (t74186, t74206, t74232, t74249, t74257, t74264, t74269, t74271, t74277)
}
