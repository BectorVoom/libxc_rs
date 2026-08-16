//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1051/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1051(t13157: f64, t4673: f64, t6060: f64, t13129: f64, t4614: f64, t813: f64, t3271: f64, t8556: f64, t2087: f64, t13037: f64, t13041: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43972 = 0.14300195980740170667e1_f64 * t6060 * t4673 * t13157;
    let t43975 = 0.61348681526273199483e1_f64 * t813 * t4614 * t13129;
    let t43977 = 0.23833659967900284446e0_f64 * t3271 * t8556;
    let t43980 = 0.82820720060468819301e2_f64 * t2087 * t4614 * t13157;
    let t43983 = 0.12269736305254639897e2_f64 * t813 * t4614 * t13037;
    let t43986 = 0.58281247449959539508e2_f64 * t833 * t4614 * t13041;
    (t43972, t43975, t43977, t43980, t43983, t43986)
}
