//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1235/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1235<F: Float>(t1048: F, t41362: F, t41375: F, t41389: F, t41401: F, t41416: F, t41429: F, t41443: F, t41456: F, t41471: F, t41484: F, t41498: F, t41511: F, t41526: F, t41537: F, t41551: F, t41564: F, t41579: F, t41592: F, t41606: F, t41619: F, t41633: F, t41646: F, t41660: F, t41673: F, t41688: F, t41700: F, t41714: F, t41727: F, t41741: F, t41754: F, t41766: F, t41777: F, t499: F, t797: F) -> F {
    let t41786 = t1048 * t499 * (t41471 + t41606 + t41766 + t41592 + t41727 + t41688 + t41526 + t41416 + t41362 + t41456 + t41714 + t41401 + t41443 + t41633 + t41564 + t41700 + t41579 + t41754 + t41375 + t41551 + t41511 + t41389 + t41741 + t41619 + t41660 + t41484 + t41646 + t41498 + t41673 + t41429 + t41777 + t41537) * t797 / F::cast_from(4.0_f64);
    t41786
}
