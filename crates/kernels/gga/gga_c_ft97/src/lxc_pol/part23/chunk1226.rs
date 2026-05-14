//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1226/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1226<F: Float>(t4960: F, t684: F, t1127: F, t6793: F, t224: F, t2427: F, t30674: F, t230: F, t5009: F, t108685: F, t6832: F, t6055: F, t108810: F, t109030: F, t109038: F, t109055: F, t109069: F, t1096: F, t112223: F, t122840: F, t13443: F, t17896: F, t17900: F, t17908: F, t18024: F, t21145: F, t231: F, t24276: F, t24324: F, t24346: F, t25057: F, t27487: F, t27522: F, t27596: F, t27642: F, t27665: F, t27699: F, t27723: F, t30641: F, t30712: F, t3691: F, t3817: F, t420: F, t6029: F, t6034: F, t6045: F, t709: F, t7477: F, t79485: F, t79641: F, t96424: F, t96593: F) -> (F, F, F, F) {
    let t123343 = t4960 * t684;
    let t123352 = t6793 * t1127;
    let t123361 = t224 * t2427 * t30674;
    let t123362 = t230 * t5009;
    let t123367 = t108685 * t6832;
    let t123368 = t6055 * t123367;
    let t123384 = -0.88910709717637694816e-2 * t13443 * t25057 * t27723 * t3817 - t109030 + 0.49489226297715094073e-4 * t109038 + t109055 - 0.27568129967481981592e-3 * t30641 * t27596 * t21145 + 0.64109413167231678972e-5 * t79641 * t27699 * t21145 - 0.11491849508333333333e0 * t24324 * t6045 * t231 * t79485 - 0.14846767889314528222e-4 * t24276 * t96424 * t123343 - 0.21120586720831816188e-4 * t108810 * t122840 * t1096 * t27522 + 0.85124811172839506173e-2 * t109069 - 0.2108030480665075738e-3 * t7477 * t112223 * t123352 * t709 - 0.11877414311451622578e-2 * t6034 * t27642 * t27665 + 0.20834636627556862176e-5 * t123361 * t420 * t123362 * t709 + 0.1134997482304526749e-1 * t123368 + 0.19795690519086037629e-3 * t24276 * t96593 * t1096 * t3691 + 0.23254900946437792e-1 * t24346 * t17908 + 0.44455354858818847408e-2 * t30712 * t6029 + 0.38731446812548799881e-3 * t27487 * t17896 - 0.38731446812548799881e-3 * t24346 * t17900 - 0.23238868087529279928e-3 * t27487 * t18024;
    (t123343, t123362, t123367, t123384)
}
