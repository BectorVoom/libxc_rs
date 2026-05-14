//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 757/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk757<F: Float>(t12871: F, t8158: F, t41878: F, t6717: F, t6914: F, t10532: F, t10533: F, t40372: F, t40377: F, t40392: F, t40395: F, t41839: F, t6710: F, t6711: F, t204: F, t587: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42161 = 0.10725146985555128001e1 * t8158 * t12871;
    let t42163 = t6914 * t6717 * t41878;
    let t42166 = t10532 * t10533 * t41878;
    let t42168 = 0.63904876589867916127e-1 * t40372;
    let t42170 = 0.19171462976960374838e0 * t40377;
    let t42172 = 0.15337170381568299871e1 * t40392;
    let t42173 = 0.29792074959875355558e-1 * t40395;
    let t42176 = t6710 * t6711 * t41839;
    let t42180 = t587 * t204 * t41878;
    (t42161, t42163, t42166, t42168, t42170, t42172, t42173, t42176, t42180)
}
