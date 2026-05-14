//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 943/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk943<F: Float>(t25528: F, t6421: F, t28: F, t108: F, t4436: F, t22883: F, t4589: F, t5710: F, t26061: F, t979: F, t23038: F, t25913: F, t25940: F, t25946: F, t25948: F, t29623: F, t29627: F, t29631: F, t29635: F, t29639: F, t29643: F, t29647: F, t29654: F, t29658: F, t29662: F, t29666: F) -> (F, F, F, F, F, F, F, F) {
    let t29744 = t25528 * t6421;
    let t29745 = t28 * t29744;
    let t29748 = t108 * t4436;
    let t29749 = t22883 * t29748;
    let t29750 = t28 * t29749;
    let t29756 = t5710 * t4589;
    let t29758 = t26061 * t979;
    let t29775 = -4.0 / 3.0 * t25913 + t29623 / 3.0 + 2.0 / 9.0 * t29627 + 2.0 / 3.0 * t29631 - t29635 / 2.0 - t29639 / 3.0 - 2.0 / 3.0 * t29643 - t29647 / 6.0 + 2.0 / 3.0 * t25940 - 2.0 / 9.0 * t25946 - t25948 / 9.0 - 2.0 / 3.0 * t29654 - t23038 + t29658 / 3.0 + t29662 / 6.0 + t29666 / 9.0;
    (t29744, t29745, t29748, t29749, t29750, t29756, t29758, t29775)
}
