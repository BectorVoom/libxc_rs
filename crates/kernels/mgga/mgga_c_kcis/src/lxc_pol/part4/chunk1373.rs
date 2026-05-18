//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1373/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1373<F: Float>(t17376: F, t17425: F, t17483: F, t17706: F, t1506: F, t1628: F, t6220: F, t2128: F, t4481: F, t4314: F, t6188: F, t1615: F) -> (F, F, F, F) {
    let t17708 = t17376 + t17425 + t17483 + t17706;
    let t17709 = t1506 * t17708;
    let t17710 = t6220 * t1628;
    let t17713 = t2128 * t4481;
    let t17730 = t6188 * t4314;
    let t17731 = t17730 * t1615;
    (t17709, t17710, t17713, t17731)
}
