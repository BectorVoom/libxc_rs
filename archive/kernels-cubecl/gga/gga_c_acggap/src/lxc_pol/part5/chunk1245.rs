//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1245/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1245<F: Float>(t1861: F, t3237: F, t1008: F, t5529: F, t1140: F, t5676: F, t17167: F, t176: F, t20305: F, t322: F, t8790: F, t5534: F) -> (F, F, F, F, F) {
    let t22848 = t3237 * t1861;
    let t22850 = t1008 * t5529;
    let t22865 = t1140 * t5676;
    let t22880 = t17167 * t176 * t8790 * t20305 * t322;
    let t22882 = t1008 * t5534;
    (t22848, t22850, t22865, t22880, t22882)
}
