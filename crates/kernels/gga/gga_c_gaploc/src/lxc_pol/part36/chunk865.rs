//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 865/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk865<F: Float>(t13129: F, t4614: F, t813: F, t3271: F, t8556: F, t13157: F, t2087: F, t13037: F, t13041: F, t833: F, t3025: F, t3255: F, t4752: F, t33232: F, t787: F, t9824: F) -> (F, F, F, F, F, F, F) {
    let t43975 = 0.61348681526273199483e1 * t813 * t4614 * t13129;
    let t43977 = 0.23833659967900284446e0 * t3271 * t8556;
    let t43980 = 0.82820720060468819301e2 * t2087 * t4614 * t13157;
    let t43983 = 0.12269736305254639897e2 * t813 * t4614 * t13037;
    let t43986 = 0.58281247449959539508e2 * t833 * t4614 * t13041;
    let t43989 = 0.7150097990370085334e0 * t3025 * t4752 * t3255;
    let t43991 = t787 * t33232 * t9824;
    (t43975, t43977, t43980, t43983, t43986, t43989, t43991)
}
