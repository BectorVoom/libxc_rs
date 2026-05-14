//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1288/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1288<F: Float>(t29476: F, t29478: F, t29480: F, t29483: F, t32597: F, t32604: F, t32610: F, t32615: F, t32618: F, t32621: F, t32623: F, t32625: F, t32629: F, t32633: F, t29486: F, t29489: F, t29492: F, t29494: F, t29498: F, t29501: F, t32635: F, t32639: F, t32642: F, t32644: F, t32646: F, t32650: F, t32653: F, t32657: F, t32661: F) -> (F, F) {
    let t39467 = -t29476 + t32597 - t32604 - t32610 + t32615 + t32618 + t32621 - t32623 - t32625 - t32629 - t32633 - t29478 + t29480 + t29483;
    let t39468 = t29486 - t29489 - t32635 - t32639 + t32642 + t32644 + t32646 + t32650 - t32653 + t32657 + t32661 + t29492 + t29494 + t29498 - t29501;
    (t39467, t39468)
}
