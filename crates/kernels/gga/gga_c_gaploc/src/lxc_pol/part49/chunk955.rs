//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 955/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk955<F: Float>(t47338: F, t780: F, t13858: F, t2194: F, t47143: F, t825: F, t969: F, t2365: F, t39149: F, t7390: F, t43686: F, t43690: F, t43693: F, t43695: F, t43698: F, t43699: F, t43708: F, t43712: F) -> (F,) {
    let t47340 = 0.35750489951850426669e0 * t780 * t47338;
    let t47341 = t2194 * t13858;
    let t47344 = t825 * t969 * t47143;
    let t47347 = t7390 * t2365 * t39149;
    let t47352 = t47340 - 0.46011511144704899612e1 * t47341 - 0.19171462976960374838e0 * t47344 + 0.14896037479937677779e-1 * t47347 - t43686 + 0.71500979903700853338e0 * t43690 + t43693 - t43695 - t43698 + 0.71500979903700853338e0 * t43699 + t43708 + 0.19171462976960374838e0 * t43712;
    (t47352,)
}
