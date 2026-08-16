//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1235/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1235(t1048: f64, t41362: f64, t41375: f64, t41389: f64, t41401: f64, t41416: f64, t41429: f64, t41443: f64, t41456: f64, t41471: f64, t41484: f64, t41498: f64, t41511: f64, t41526: f64, t41537: f64, t41551: f64, t41564: f64, t41579: f64, t41592: f64, t41606: f64, t41619: f64, t41633: f64, t41646: f64, t41660: f64, t41673: f64, t41688: f64, t41700: f64, t41714: f64, t41727: f64, t41741: f64, t41754: f64, t41766: f64, t41777: f64, t499: f64, t797: f64) -> f64 {
    let t41786 = t1048 * t499 * (t41471 + t41606 + t41766 + t41592 + t41727 + t41688 + t41526 + t41416 + t41362 + t41456 + t41714 + t41401 + t41443 + t41633 + t41564 + t41700 + t41579 + t41754 + t41375 + t41551 + t41511 + t41389 + t41741 + t41619 + t41660 + t41484 + t41646 + t41498 + t41673 + t41429 + t41777 + t41537) * t797 / 4.0_f64;
    t41786
}
