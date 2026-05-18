//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 876/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk876<F: Float>(t2145: F, t27: F, t5249: F, t649: F, t39157: F, t39162: F, t39167: F, t39172: F, t39177: F, t39181: F, t39184: F, t39189: F, t39193: F, t39197: F, t39200: F, t39205: F, t39209: F, t39215: F, t39219: F, t39224: F) -> F {
    let t39228 = t2145 * t27 * t649 * t5249;
    let t39230 = -F::new(0.51077519871957407276e-4) * t39157 + F::new(0.76616279807936110914e-4) * t39162 + F::new(0.25538759935978703638e-4) * t39167 - F::new(0.25538759935978703638e-4) * t39172 + F::new(0.31923449919973379548e-4) * t39177 + F::new(0.76616279807936110914e-4) * t39181 - F::new(0.76616279807936110914e-4) * t39184 + F::new(0.31923449919973379548e-4) * t39189 - F::new(0.15323255961587222183e-3) * t39193 - F::new(0.51077519871957407276e-4) * t39197 + F::new(0.51077519871957407276e-4) * t39200 + F::new(0.95770349759920138643e-4) * t39205 + F::new(0.1064114997332445985e-4) * t39209 - F::new(0.12769379967989351819e-4) * t39215 - F::new(0.42564599893297839398e-5) * t39219 - F::new(0.212822999466489197e-4) * t39224 - F::new(0.34093327067806677161e-2) * t39228;
    t39230
}
