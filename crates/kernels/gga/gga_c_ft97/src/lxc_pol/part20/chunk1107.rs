//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1107/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1107<F: Float>(t109216: F, t109221: F, t109231: F, t109245: F, t109247: F, t109254: F, t1096: F, t13443: F, t13460: F, t13505: F, t13516: F, t13522: F, t14832: F, t1701: F, t2413: F, t24261: F, t24276: F, t24278: F, t2428: F, t24346: F, t24382: F, t24386: F, t27494: F, t27500: F, t27704: F, t27733: F, t35410: F, t3751: F, t3766: F, t66423: F, t684: F, t96739: F) -> (F,) {
    let t109257 = -t96739 + 0.46509801892875584e-1 * t24346 * t13516 + 0.23254900946437792e-1 * t24346 * t13460 + 0.23254900946437792e-1 * t24346 * t13505 - 0.23254900946437792e-1 * t27704 * t24382 + 0.38731446812548799881e-3 * t27704 * t24386 + 4.0 * t3766 * t109216 * t2428 - 0.51074886703703703704e-1 * t27500 * t109221 + 4.0 * t27733 * t24261 + 0.2370952259137005195e-1 * t13443 * t1701 * t27494 * t2428 - 0.17816121467177433866e-2 * t109231 * t35410 * t13522 + 0.14846767889314528222e-3 * t24276 * t24278 * t3751 * t684 + 0.7423383944657264111e-4 * t24276 * t24278 * t1096 * t2413 - 0.34526011664076264184e-5 * t109245 * t109247 * t14832 * t684 - 0.35625083901748972663e-8 * t66423 * t109254;
    (t109257,)
}
