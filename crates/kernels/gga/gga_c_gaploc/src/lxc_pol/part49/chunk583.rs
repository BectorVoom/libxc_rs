//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 583/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk583<F: Float>(t2375: F, t8248: F, t8229: F, t901: F, t8331: F, t2413: F, t8411: F, t10241: F, t475: F, t6717: F, t6914: F, t6711: F, t2487: F, t204: F, t587: F, t1: F, t1559: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10506 = 0.11916829983950142223e0 * t8248 * t2375;
    let t10507 = t8229 * t901;
    let t10508 = 0.14896037479937677779e-1 * t10507;
    let t10509 = t8331 * t901;
    let t10510 = 0.14896037479937677779e-1 * t10509;
    let t10512 = 0.10725146985555128001e1 * t8411 * t2413;
    let t10513 = t10241 * t475;
    let t10514 = t6717 * t10513;
    let t10516 = 0.62115540045351614476e2 * t6914 * t10514;
    let t10517 = t6711 * t10513;
    let t10519 = 0.43710935587469654631e2 * t2487 * t10517;
    let t10520 = t204 * t10513;
    let t10522 = 0.92023022289409799224e1 * t587 * t10520;
    let t10523 = t1559 * t1;
    (t10506, t10508, t10510, t10512, t10513, t10516, t10519, t10522, t10523)
}
