//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1341/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1341<F: Float>(t34943: F, t10438: F, t1391: F, t587: F, t10618: F, t20957: F, t20671: F, t20688: F, t26435: F, t31207: F, t10532: F, t10533: F, t34239: F) -> (F, F, F, F, F, F) {
    let t34944 = F::new(0.51123901271894332902e0) * t34943;
    let t34946 = t587 * t1391 * t10438;
    let t34947 = F::new(0.2698205900461089792e0) * t34946;
    let t34949 = t20957 * t10618;
    let t34950 = F::new(0.29792074959875355558e-1) * t34949;
    let t34952 = t20688 * t20671 * t26435;
    let t34953 = F::new(0.85206502119823888168e-1) * t34952;
    let t34954 = F::new(0.31952438294933958064e-1) * t31207;
    let t34957 = F::new(0.55213813373645879534e2) * t10532 * t10533 * t34239;
    (t34944, t34947, t34950, t34953, t34954, t34957)
}
