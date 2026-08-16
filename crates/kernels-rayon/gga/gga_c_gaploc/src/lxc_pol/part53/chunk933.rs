//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 933/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk933(t13157: f64, t2087: f64, t4614: f64, t13037: f64, t813: f64, t13041: f64, t833: f64, t3025: f64, t3255: f64, t4752: f64, t41405: f64, t41408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43980 = 0.82820720060468819301e2_f64 * t2087 * t4614 * t13157;
    let t43983 = 0.12269736305254639897e2_f64 * t813 * t4614 * t13037;
    let t43986 = 0.58281247449959539508e2_f64 * t833 * t4614 * t13041;
    let t43989 = 0.7150097990370085334e0_f64 * t3025 * t4752 * t3255;
    let t43993 = 0.20854452471912748891e0_f64 * t41405;
    let t43994 = 0.19171462976960374838e0_f64 * t41408;
    (t43980, t43983, t43986, t43989, t43993, t43994)
}
