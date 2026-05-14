//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1344/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1344<F: Float>(t20409: F, t20426: F, t20435: F, t20439: F, t20445: F, t20446: F, t20452: F, t20457: F, t22836: F, t25357: F, t25361: F, t25365: F, t25369: F, t25372: F, t25379: F, t551: F, t552: F) -> (F,) {
    let t25387 = -0.1047928639570397803e0 * t25357 - 0.41917145582815912122e0 * t25361 - 0.69861909304693186866e-1 * t25365 + 0.20803732176130244552e1 * t25369 - 0.97574405393827830186e-3 * t20409 + 0.15602799132097683414e2 * t22836 * t551 * t552 * t25372 + 0.20803732176130244552e2 * t25379 - 0.77115101645255404583e-4 * t20426 - 0.38087975358139160777e-1 * t20435 - 0.19043987679069580389e-1 * t20439 + t20445 - 0.35126785941778018867e0 * t20446 + 0.48787202696913915094e-3 * t20452 + 0.29272321618148349056e-1 * t20457;
    (t25387,)
}
