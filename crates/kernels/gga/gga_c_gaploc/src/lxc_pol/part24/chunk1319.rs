//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1319/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1319<F: Float>(t10686: F, t107: F, t10955: F, t10958: F, t11020: F, t2021: F, t2023: F, t2194: F, t2197: F, t28726: F, t28729: F, t28731: F, t33584: F, t33586: F, t33590: F, t33604: F, t33607: F, t33610: F, t33613: F, t33616: F, t33619: F, t6159: F) -> F {
    let t33620 = -t33584 + t33586 - t28726 + t28729 + t28731 - t33590 + F::new(0.79445533226334281486e-1) * t2021 * t10686 * t107 * t2023 - F::new(0.46011511144704899612e1) * t6159 * t11020 - F::new(0.92023022289409799224e1) * t2194 * t10955 + F::new(0.23005755572352449806e2) * t2197 * t10958 - t33604 + t33607 + t33610 - t33613 + t33616 - t33619;
    t33620
}
