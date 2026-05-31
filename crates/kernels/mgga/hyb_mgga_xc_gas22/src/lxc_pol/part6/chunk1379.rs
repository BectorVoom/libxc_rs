//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1379/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1379<F: Float>(t29851: F, t29853: F, t29855: F, t29857: F, t29860: F, t29862: F, t29865: F, t29867: F, t29870: F, t29873: F, t29877: F, t29880: F) -> F {
    let t29959 = -F::cast_from(0.3529725e1_f64) * t29851 - F::cast_from(0.17648625e1_f64) * t29853 - F::cast_from(0.157790625e0_f64) * t29855 + F::cast_from(0.6311625e0_f64) * t29857 + F::cast_from(0.6311625e0_f64) * t29860 + F::cast_from(0.31558125e0_f64) * t29862 + F::cast_from(0.10589175e2_f64) * t29865 - F::cast_from(0.6311625e0_f64) * t29867 + F::cast_from(0.34731666666666666667e0_f64) * t29870 - F::cast_from(0.83356e0_f64) * t29873 + F::cast_from(0.62517e0_f64) * t29877 - F::cast_from(0.41678e0_f64) * t29880;
    t29959
}
