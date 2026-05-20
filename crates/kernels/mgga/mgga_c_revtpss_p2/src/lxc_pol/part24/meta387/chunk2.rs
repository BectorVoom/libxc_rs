//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1292/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1292<F: Float>(t24881: F, t25025: F, t12587: F, t1300: F, t1832: F, t198: F, t20692: F, t24468: F, t24478: F, t24482: F, t24484: F, t24490: F, t24492: F, t24496: F, t24500: F, t24501: F, t24763: F, t24767: F, t336: F, t5023: F) -> (F, F) {
    let t25026 = t24881 + t25025;
    let t25030 = F::new(2.0) * t12587 * t198 * t24501 * t336 + t1300 * t198 * t25026 * t336 - F::new(3.0) * t1832 * t20692 * t5023 - t24468 - t24478 - t24482 - t24484 + t24490 - t24492 + t24496 - t24500 + t24763 - t24767;
    (t25026, t25030)
}
