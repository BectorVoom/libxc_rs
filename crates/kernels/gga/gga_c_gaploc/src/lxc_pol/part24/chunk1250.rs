//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1250/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1250<F: Float>(t10618: F, t20957: F, t20671: F, t20688: F, t26435: F, t31207: F, t10532: F, t10533: F, t34239: F, t10520: F, t1407: F, t204: F, t2476: F, t34407: F, t10615: F, t30848: F) -> (F, F, F, F, F, F, F) {
    let t34949 = t20957 * t10618;
    let t34950 = 0.29792074959875355558e-1 * t34949;
    let t34952 = t20688 * t20671 * t26435;
    let t34953 = 0.85206502119823888168e-1 * t34952;
    let t34954 = 0.31952438294933958064e-1 * t31207;
    let t34957 = 0.55213813373645879534e2 * t10532 * t10533 * t34239;
    let t34959 = 0.18404604457881959845e2 * t1407 * t10520;
    let t34962 = 0.92023022289409799224e1 * t2476 * t204 * t34407;
    let t34964 = 0.50050685932590597338e1 * t10615 * t30848;
    (t34950, t34953, t34954, t34957, t34959, t34962, t34964)
}
