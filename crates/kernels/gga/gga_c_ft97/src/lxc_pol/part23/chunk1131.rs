//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1131/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1131<F: Float>(t109127: F, t6055: F, t1095: F, t1410: F, t24345: F, t27703: F, t24286: F, t6828: F, t1417: F, t236: F, t6776: F, t2426: F, t2427: F, t3758: F, t24275: F, t13411: F, t3722: F, t3724: F, t6032: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109128 = t6055 * t109127;
    let t109153 = t1410 * t1095;
    let t109159 = t27703 * t24345;
    let t109168 = t6828 * t24286;
    let t109169 = t1417 * t109168;
    let t109200 = t236 * t6776;
    let t109216 = t2426 * t6776;
    let t109230 = t3758 * t2427;
    let t109231 = t109230 * t24275;
    let t109245 = t13411 * t3722 * t3724 * t6032;
    (t109128, t109153, t109159, t109168, t109169, t109200, t109216, t109231, t109245)
}
