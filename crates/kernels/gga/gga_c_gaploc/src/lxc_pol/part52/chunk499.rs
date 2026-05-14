//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 499/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk499<F: Float>(t10477: F, t1572: F, t8063: F, t895: F, t3377: F, t8155: F, t8158: F, t2375: F, t8248: F, t8229: F, t901: F, t8331: F, t2413: F, t8411: F, t10241: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10479 = 0.71500979903700853338e0 * t1572 * t10477;
    let t10484 = 0.23833659967900284446e0 * t895 * t8063;
    let t10501 = 0.10725146985555128001e1 * t8155 * t3377;
    let t10503 = 0.10725146985555128001e1 * t8158 * t3377;
    let t10506 = 0.11916829983950142223e0 * t8248 * t2375;
    let t10507 = t8229 * t901;
    let t10508 = 0.14896037479937677779e-1 * t10507;
    let t10509 = t8331 * t901;
    let t10510 = 0.14896037479937677779e-1 * t10509;
    let t10512 = 0.10725146985555128001e1 * t8411 * t2413;
    let t10513 = t10241 * t475;
    (t10479, t10484, t10501, t10503, t10506, t10507, t10508, t10509, t10510, t10512, t10513)
}
