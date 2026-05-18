//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 933/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk933<F: Float>(t13157: F, t2087: F, t4614: F, t13037: F, t813: F, t13041: F, t833: F, t3025: F, t3255: F, t4752: F, t41405: F, t41408: F) -> (F, F, F, F, F, F) {
    let t43980 = F::new(0.82820720060468819301e2) * t2087 * t4614 * t13157;
    let t43983 = F::new(0.12269736305254639897e2) * t813 * t4614 * t13037;
    let t43986 = F::new(0.58281247449959539508e2) * t833 * t4614 * t13041;
    let t43989 = F::new(0.7150097990370085334e0) * t3025 * t4752 * t3255;
    let t43993 = F::new(0.20854452471912748891e0) * t41405;
    let t43994 = F::new(0.19171462976960374838e0) * t41408;
    (t43980, t43983, t43986, t43989, t43993, t43994)
}
