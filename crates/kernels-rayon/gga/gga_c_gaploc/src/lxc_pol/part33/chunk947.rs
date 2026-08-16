//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 947/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk947(t3377: f64, t8155: f64, t8158: f64, t2375: f64, t8248: f64, t8229: f64, t901: f64, t8331: f64, t2413: f64, t8411: f64, t10241: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10501 = 0.10725146985555128001e1_f64 * t8155 * t3377;
    let t10503 = 0.10725146985555128001e1_f64 * t8158 * t3377;
    let t10506 = 0.11916829983950142223e0_f64 * t8248 * t2375;
    let t10507 = t8229 * t901;
    let t10508 = 0.14896037479937677779e-1_f64 * t10507;
    let t10509 = t8331 * t901;
    let t10510 = 0.14896037479937677779e-1_f64 * t10509;
    let t10512 = 0.10725146985555128001e1_f64 * t8411 * t2413;
    let t10513 = t10241 * t475;
    (t10501, t10503, t10506, t10508, t10510, t10512, t10513)
}
