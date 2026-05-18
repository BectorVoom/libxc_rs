//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1040/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1040<F: Float>(t36511: F, t36513: F, t1664: F, t2127: F, t16156: F, t9055: F, t2085: F, t8339: F, t1162: F, t1979: F, t1982: F, t201: F, t589: F) -> (F, F, F, F, F, F) {
    let t41647 = F::new(0.19863479950205658386e-3) * t36511;
    let t41648 = F::new(0.19863479950205658386e-3) * t36513;
    let t41651 = t1664 * t2127;
    let t41654 = t16156 * t9055;
    let t41656 = t8339 * t2085;
    let t41657 = F::new(0.18183107769496894486e-1) * t41656;
    let t41663 = t589 * t1162 * t201 * t1979 * t1982;
    (t41647, t41648, t41651, t41654, t41657, t41663)
}
