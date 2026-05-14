//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 900/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk900<F: Float>(t10677: F, t1445: F, t2530: F, t813: F, t13157: F, t4673: F, t6060: F, t13129: F, t4614: F, t3271: F, t8556: F, t2087: F, t13037: F, t13041: F, t833: F, t3025: F, t3255: F, t4752: F) -> (F, F, F, F, F, F, F, F) {
    let t43968 = t813 * t1445 * t10677 * t2530;
    let t43972 = 0.14300195980740170667e1 * t6060 * t4673 * t13157;
    let t43975 = 0.61348681526273199483e1 * t813 * t4614 * t13129;
    let t43977 = 0.23833659967900284446e0 * t3271 * t8556;
    let t43980 = 0.82820720060468819301e2 * t2087 * t4614 * t13157;
    let t43983 = 0.12269736305254639897e2 * t813 * t4614 * t13037;
    let t43986 = 0.58281247449959539508e2 * t833 * t4614 * t13041;
    let t43989 = 0.7150097990370085334e0 * t3025 * t4752 * t3255;
    (t43968, t43972, t43975, t43977, t43980, t43983, t43986, t43989)
}
