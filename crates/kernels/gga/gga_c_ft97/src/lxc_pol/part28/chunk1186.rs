//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1186/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1186<F: Float>(t1017: F, t1023: F, t1349: F, t1360: F, t147894: F, t148196: F, t148686: F, t148703: F, t148722: F, t148726: F, t148955: F, t160: F, t26793: F, t27406: F, t28: F, t3313: F, t33221: F, t3414: F, t5778: F, t5973: F, t7309: F, t7412: F) -> F {
    let t149630 = -F::new(2.0) * t147894 + F::new(2.0) * t148686 * t160 - F::new(2.0) / F::new(3.0) * t1349 * t28 * t5778 * t5973 * t1017 - t7309 * t26793 / F::new(3.0) - F::new(4.0) * t148726 - t3313 * t7412 - F::new(2.0) * t148722 - t1023 * t33221 - t3414 * t7412 - F::new(4.0) * t148703 - F::new(4.0) * t148196 + F::new(4.0) * t148955 + t1349 * t28 * t1360 * t27406 / F::new(3.0);
    t149630
}
