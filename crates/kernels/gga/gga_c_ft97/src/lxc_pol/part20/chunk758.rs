//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 758/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk758<F: Float>(t13580: F, t24389: F, t13407: F, t13413: F, t13443: F, t1417: F, t1701: F, t2389: F, t24332: F, t24337: F, t24342: F, t24346: F, t24349: F, t24353: F, t24358: F, t24361: F, t24363: F, t24367: F, t24372: F, t24374: F, t24380: F, t24382: F, t24386: F, t3759: F, t3766: F, t3774: F, t6023: F, t6034: F, t6035: F, t9543: F) -> (F,) {
    let t24390 = t13580 * t24389;
    let t24393 = 0.25537443351851851852e-1 * t24332 + 0.25845121844514357744e-4 * t3774 * t6023 * t13407 - 2.0 * t3766 * t24337 - 0.60102574844279699039e-6 * t13413 * t24342 + 0.46509801892875584e-1 * t24346 * t2389 - 0.44455354858818847408e-2 * t13443 * t1701 * t24349 + 0.22227677429409423704e-2 * t1417 * t1701 * t24353 - 0.42562405586419753086e-2 * t24358 + 0.25537443351851851852e-1 * t24361 * t6035 * t24363 + 0.22270151833971792333e-3 * t6034 * t6035 * t24367 - 0.14836531933660919214e-4 * t24372 * t6035 * t24374 + 0.14846767889314528222e-3 * t24380 - 0.23254900946437792e-1 * t3759 * t24382 + 0.38731446812548799881e-3 * t3759 * t24386 + 0.13519760450715832853e-3 * t9543 * t24390;
    (t24393,)
}
