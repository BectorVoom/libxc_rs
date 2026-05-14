//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1235/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1235<F: Float>(t4635: F, t679: F, t1411: F, t27703: F, t108503: F, t108860: F, t108880: F, t109109: F, t109159: F, t122803: F, t123343: F, t123424: F, t123560: F, t123681: F, t123697: F, t123700: F, t123709: F, t123713: F, t123716: F, t13520: F, t17864: F, t17933: F, t237: F, t24276: F, t24346: F, t25: F, t27500: F, t27537: F, t27605: F, t27699: F, t30760: F, t30763: F, t3723: F, t3730: F, t3734: F, t3755: F, t3759: F, t3762: F, t3786: F, t6014: F, t6023: F, t6035: F, t6055: F, t66076: F, t677: F, t684: F, t689: F, t79942: F, t96451: F, t96600: F, t96602: F) -> (F,) {
    let t123724 = t4635 * t679;
    let t123740 = t27703 * t1411;
    let t123747 = -0.10357803499222879255e-4 * t123681 * t108860 - 0.12255510004984495842e-5 * t66076 * t27605 * t17864 - 0.25845121844514357744e-4 * t13520 * t6023 * t79942 - 0.23254900946437792e-1 * t3759 * t6014 * t123560 + 0.27039520901431665705e-3 * t3723 * t108880 * t27699 + 0.51074886703703703703e-1 * t123697 - 0.6809984893827160494e-1 * t6055 * t123700 - 0.25845121844514357744e-4 * t13520 * t6023 * t123424 + 0.46509801892875584e-1 * t109109 * t3786 - 0.19862455940329218107e-1 * t27500 * t123709 + 0.34049924469135802469e-1 * t27500 * t123713 - 0.38306165027777777777e-1 * t96451 * t6035 * t123716 * t684 - 0.4945510644553639738e-5 * t96600 * t96602 * t123343 + 0.74233839446572641111e-4 * t24276 * t27537 * t123724 * t689 + 0.98978452595430188147e-4 * t24276 * t108503 * t122803 + 0.44745149797750190322e-9 * t677 * t237 * t30760 * t30763 * t25 * t3762 + 0.46509801892875584e-1 * t109159 * t3730 + 0.93019603785751168e-2 * t123740 * t3734 + 0.77462893625097599762e-3 * t123740 * t3755 - 0.46509801892875584e-2 * t24346 * t17933;
    (t123747,)
}
