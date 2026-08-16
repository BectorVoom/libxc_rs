//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 636/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk636(t2375: f64, t8248: f64, t8229: f64, t901: f64, t8331: f64, t2413: f64, t8411: f64, t10241: f64, t475: f64, t6717: f64, t6914: f64, t6711: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10506 = 0.11916829983950142223e0_f64 * t8248 * t2375;
    let t10507 = t8229 * t901;
    let t10508 = 0.14896037479937677779e-1_f64 * t10507;
    let t10509 = t8331 * t901;
    let t10510 = 0.14896037479937677779e-1_f64 * t10509;
    let t10512 = 0.10725146985555128001e1_f64 * t8411 * t2413;
    let t10513 = t10241 * t475;
    let t10514 = t6717 * t10513;
    let t10516 = 0.62115540045351614476e2_f64 * t6914 * t10514;
    let t10517 = t6711 * t10513;
    (t10506, t10508, t10510, t10512, t10513, t10516, t10517)
}
