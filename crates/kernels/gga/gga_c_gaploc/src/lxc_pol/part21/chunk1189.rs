//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1189/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1189<F: Float>(t1367: F, t20901: F, t35180: F, t10547: F, t6820: F, t204: F, t2476: F, t34411: F, t34407: F, t6710: F, t6711: F, t1429: F, t2365: F, t2366: F, t7861: F, t18970: F, t3381: F) -> (F, F, F, F, F, F) {
    let t35183 = 0.55611873258433997041e0 * t35180 * t20901 * t1367;
    let t35185 = 0.25025342966295298669e1 * t10547 * t6820;
    let t35188 = 0.46011511144704899612e1 * t2476 * t204 * t34411;
    let t35192 = 0.23005755572352449806e2 * t6710 * t6711 * t34407;
    let t35198 = t1429 * t2365 * t2366 * t7861;
    let t35199 = 0.14896037479937677779e-1 * t35198;
    let t35200 = t18970 * t3381;
    (t35183, t35185, t35188, t35192, t35199, t35200)
}
