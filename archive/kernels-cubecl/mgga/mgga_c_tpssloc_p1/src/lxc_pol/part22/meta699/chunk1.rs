//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2283/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2283<F: Float>(t15643: F, t5024: F, t19201: F, t3576: F, t3577: F, t44951: F, t6191: F, t13969: F, t19061: F, t3515: F, t15568: F, t5064: F) -> (F, F, F, F, F) {
    let t65803 = t5024 * t15643;
    let t65815 = t19201 * t3576;
    let t65819 = t3577 * t44951 * t6191;
    let t65881 = t3515 * t13969 * t19061;
    let t65884 = t5064 * t15568;
    (t65803, t65815, t65819, t65881, t65884)
}
