//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 915/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk915<F: Float>(t39147: F, t39157: F, t39162: F, t39167: F, t39172: F, t39177: F, t39181: F, t39184: F, t39189: F, t39193: F, t39197: F, t39200: F, t39205: F, t39209: F, t39215: F, t39219: F, t39224: F, t39228: F) -> (F,) {
    let t42880 = -0.1276937996798935182e-3 * t39147 - 0.10215503974391481456e-3 * t39157 + 0.15323255961587222184e-3 * t39162 + 0.5107751987195740728e-4 * t39167 - 0.5107751987195740728e-4 * t39172 + 0.638468998399467591e-4 * t39177 + 0.15323255961587222184e-3 * t39181 - 0.15323255961587222184e-3 * t39184 + 0.638468998399467591e-4 * t39189 - 0.30646511923174444368e-3 * t39193 - 0.10215503974391481456e-3 * t39197 + 0.10215503974391481456e-3 * t39200 + 0.1915406995198402773e-3 * t39205 + 0.212822999466489197e-4 * t39209 - 0.2553875993597870364e-4 * t39215 - 0.85129199786595678799e-5 * t39219 - 0.425645998932978394e-4 * t39224 - 0.68186654135613354325e-2 * t39228;
    (t42880,)
}
