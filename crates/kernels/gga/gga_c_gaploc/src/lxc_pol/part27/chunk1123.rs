//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1123/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1123<F: Float>(t33386: F, t10883: F, t7416: F, t1980: F, t8788: F, t9824: F, t22424: F, t3500: F, t2975: F, t6134: F, t7372: F, t1445: F, t2087: F, t24926: F, t935: F, t10820: F, t10914: F, t2089: F, t539: F) -> (F, F, F, F, F, F, F) {
    let t33387 = 0.38342925953920749676e0 * t33386;
    let t33388 = t7416 * t10883;
    let t33389 = 0.85206502119823888168e-1 * t33388;
    let t33391 = t1980 * t8788 * t9824;
    let t33392 = 0.29792074959875355558e-1 * t33391;
    let t33393 = t22424 * t3500;
    let t33394 = 0.19171462976960374838e0 * t33393;
    let t33396 = t6134 * t2975 * t7372;
    let t33397 = 0.29792074959875355558e-1 * t33396;
    let t33405 = 0.69017266717057349418e1 * t2087 * t1445 * t24926 * t935;
    let t33409 = 0.28600391961480341335e1 * t10914 * t539 * t2089 * t10820;
    (t33387, t33389, t33392, t33394, t33397, t33405, t33409)
}
