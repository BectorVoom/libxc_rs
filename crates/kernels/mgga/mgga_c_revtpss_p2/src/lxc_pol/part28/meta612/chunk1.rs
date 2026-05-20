//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2138/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2138<F: Float>(t28189: F, t7235: F, t2014: F, t7900: F, t94358: F, t10416: F, t13435: F, t7746: F, t98522: F, t98525: F, t98528: F, t98530: F, t98532: F, t98534: F, t98537: F, t98539: F, t98541: F, t98544: F, t98546: F, t98549: F, t98553: F, t98555: F, t98557: F) -> F {
    let t98559 = F::new(2.0) * t7235 * t28189;
    let t98562 = F::new(3.0) * t2014 * t94358 * t7900;
    let t98563 = -F::new(2.0) * t10416 * t7746 - F::new(4.0) * t13435 * t7746 - t98522 + t98525 - t98528 + t98530 - t98532 - t98534 - t98537 - t98539 + t98541 - t98544 + t98546 + t98549 + t98553 + t98555 + t98557 - t98559 + t98562;
    t98563
}
