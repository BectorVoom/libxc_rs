//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1006/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1006<F: Float>(t2205: F, t6849: F, t311: F, t103: F, t314: F, t7875: F, t7158: F, t7591: F, t8134: F, t7877: F, t875: F, t15512: F, t966: F) -> (F, F, F, F, F, F) {
    let t17890 = F::new(1.0) / t6849 / t2205;
    let t17891 = t311 * t17890;
    let t17899 = t7875 * t314 * t103;
    let t18018 = t7591 * t7158 * t8134;
    let t18105 = t875 * t7877;
    let t18107 = t15512 * t966 * t18105 * t103;
    (t17890, t17891, t17899, t18018, t18105, t18107)
}
