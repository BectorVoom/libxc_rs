//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1313/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1313<F: Float>(t108: F, t580: F, t22: F, t4283: F, t105: F, t13472: F, t13475: F, t13476: F, t13479: F, t13482: F, t13485: F, t13493: F, t13496: F, t13497: F, t13500: F, t1505: F, t1507: F, t2344: F, t2359: F, t2363: F, t4270: F, t4274: F, t656: F, t97: F) -> F {
    let t13503 = t108 * t580;
    let t13506 = t4283 * t22;
    let t13509 = F::new(200.0) / F::new(27.0) * t2344 * t1505 - F::new(100.0) / F::new(27.0) * t656 * t4270 - F::new(50.0) / F::new(9.0) * t656 * t4274 - F::new(10.0) / F::new(27.0) * t97 * t13472 + F::new(20.0) / F::new(9.0) * t13475 * t13476 + F::new(10.0) / F::new(9.0) * t97 * t13479 + F::new(5.0) / F::new(3.0) * t97 * t13482 - F::new(5.0) * t97 * t13485 - F::new(50.0) / F::new(27.0) * t1507 * t2359 - F::new(25.0) / F::new(9.0) * t1507 * t2363 - F::new(10.0) / F::new(27.0) * t105 * t13493 - F::new(20.0) / F::new(9.0) * t13496 * t13497 + F::new(10.0) / F::new(9.0) * t105 * t13500 - F::new(5.0) / F::new(3.0) * t105 * t13503 + F::new(5.0) * t105 * t13506;
    t13509
}
