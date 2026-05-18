//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1157/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1157<F: Float>(t36986: F, t42830: F, t1065: F, t2530: F, t3270: F, t3579: F, t3060: F, t36967: F, t3269: F, t10615: F, t12395: F, t3262: F) -> (F, F, F, F) {
    let t42832 = F::new(3.0) / F::new(2.0) * t36986 * t42830;
    let t42836 = t3579 * t3270 * t1065 * t2530 / F::new(2.0);
    let t42837 = t1065 * t3060;
    let t42838 = t36967 * t42837;
    let t42840 = F::new(45.0) / F::new(64.0) * t3269 * t42838;
    let t42843 = F::new(15.0) / F::new(8.0) * t3262 * t10615 * t12395;
    (t42832, t42836, t42840, t42843)
}
