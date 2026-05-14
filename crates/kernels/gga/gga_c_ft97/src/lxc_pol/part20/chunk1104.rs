//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1104/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1104<F: Float>(t108685: F, t6056: F, t6055: F, t24290: F, t27506: F, t27511: F, t3626: F, t6044: F, t6832: F, t96535: F, t1095: F, t1410: F, t13659: F, t13662: F, t2035: F, t231: F, t24260: F, t2428: F, t24346: F, t27487: F, t27717: F, t3766: F, t6045: F, t65676: F, t66383: F, t66416: F, t66612: F, t66619: F, t695: F, t6979: F, t709: F, t7477: F, t96442: F, t96510: F, t96723: F, t96726: F) -> (F, F, F, F, F) {
    let t109117 = t108685 * t6056;
    let t109119 = 0.1134997482304526749e-1 * t6055 * t109117;
    let t109120 = t27506 * t24290;
    let t109124 = t6044 * t3626 * t27511;
    let t109125 = t6055 * t109124;
    let t109127 = t96535 * t6832;
    let t109128 = t6055 * t109127;
    let t109153 = t1410 * t1095;
    let t109158 = -0.28374937057613168724e-2 * t96723 - 0.21281202793209876543e-2 * t96726 + t109119 - 0.3404992446913580247e-1 * t6055 * t109120 - 0.2979368391049382716e-1 * t109125 + 0.14187468528806584362e-2 * t109128 + 0.45967398033333333332e0 * t96442 * t6045 * t231 * t65676 - 0.46509801892875584e-2 * t24346 * t13659 - 0.1054015240332537869e-3 * t7477 * t2035 * t6979 * t2428 - 0.558117622714507008e-2 * t27487 * t13662 + 8.0 * t3766 * t24260 * t66612 + 4.0 * t3766 * t24260 * t66619 - 12.0 * t3766 * t96510 * t65676 - 0.11854761295685025975e-1 * t27717 * t66416 - 0.27039520901431665706e-3 * t66383 * t695 * t109153 * t709;
    (t109117, t109120, t109124, t109127, t109158)
}
