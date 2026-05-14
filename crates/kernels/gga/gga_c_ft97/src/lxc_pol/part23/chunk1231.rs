//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1231/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1231<F: Float>(t123015: F, t6056: F, t24330: F, t30775: F, t6055: F, t14: F, t30674: F, t3771: F, t694: F, t108606: F, t109153: F, t109159: F, t1095: F, t1113: F, t112223: F, t1127: F, t123538: F, t123543: F, t123552: F, t123560: F, t123565: F, t123579: F, t123582: F, t17806: F, t17807: F, t17819: F, t17994: F, t213: F, t231: F, t232: F, t25057: F, t27521: F, t27523: F, t27546: F, t27561: F, t27596: F, t27605: F, t27609: F, t27695: F, t27711: F, t30676: F, t30727: F, t30853: F, t36796: F, t3759: F, t3774: F, t3786: F, t3817: F, t6023: F, t6045: F, t66383: F, t6819: F, t690: F, t695: F, t6979: F, t709: F, t79529: F, t96576: F) -> (F, F, F) {
    let t123591 = t123015 * t6056;
    let t123594 = t24330 * t30775;
    let t123595 = t6055 * t123594;
    let t123599 = t3771 * t694 * t30674 * t14;
    let t123603 = 0.1836608226397146721e-4 * t3774 * t108606 * t27561 + 0.18164417702296932716e-2 * t27521 * t6819 * t231 * t213 * t3817 - 0.10595910326339877418e-1 * t27521 * t30727 * t27523 + 0.2108030480665075738e-3 * t36796 * t112223 * t123538 * t709 - 0.2370952259137005195e-1 * t123543 * t17994 + 0.46509801892875584e-1 * t109159 * t3786 - 0.27039520901431665705e-3 * t66383 * t695 * t109153 * t1127 + 0.44455354858818847408e-2 * t27711 * t25057 * t123552 * t709 + 0.60102574844279699039e-6 * t30853 * t96576 + 0.25845121844514357744e-4 * t3774 * t6023 * t123560 - 0.60102574844279699039e-6 * t17819 * t123565 + 0.12255510004984495842e-5 * t17807 * t27605 * t27596 - 0.71250167803497945327e-8 * t79529 * t17806 * t6979 * t1095 - 0.46509801892875584e-1 * t3759 * t27695 * t27596 + 0.3520097786805302698e-5 * t123579 + 0.44540303667943584666e-3 * t27609 * t232 * t123582 + 0.15322466011111111111e0 * t27546 * t6045 * t231 * t1113 * t3817 - 0.62424861526748971193e-1 * t6055 * t123591 + 0.42562405586419753087e-2 * t123595 + 0.12112685275721489028e-7 * t123599 * t30676 * t690;
    (t123591, t123594, t123603)
}
