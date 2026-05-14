//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1027/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1027<F: Float>(t43506: F, t43519: F, t43534: F, t43537: F, t43933: F, t43936: F, t43503: F, t43511: F, t43516: F, t43522: F, t43528: F, t43531: F, t43930: F, t43940: F, t10238: F, t2649: F, t2745: F, t2892: F, t317: F, t44131: F, t44262: F, t44272: F, t44352: F, t44362: F, t44483: F, t44603: F, t44718: F, t44736: F, t44751: F, t44767: F, t788: F, t829: F, t880: F) -> (F,) {
    let t44769 = 4.0 / 27.0 * t43506;
    let t44771 = 8.0 / 9.0 * t43519;
    let t44775 = 8.0 / 81.0 * t43534;
    let t44776 = 140.0 / 243.0 * t43537;
    let t44778 = 56.0 / 81.0 * t43933;
    let t44779 = 4.0 / 9.0 * t43936;
    let t44781 = -6.0 * t43503 - t44769 - 4.0 / 3.0 * t43511 + t43516 + t44771 + 4.0 / 3.0 * t43522 + 4.0 * t43528 + 2.0 / 9.0 * t43531 - t44775 + t44776 - t43930 / 6.0 + t44778 - t44779 + 4.0 / 9.0 * t43940;
    let t44789 = -6.0 * t2745 * t2892 - 8.0 * t10238 * t880 - 12.0 * t44272 - 8.0 * t44483 - 6.0 * t2649 * t2892 - t44718 * t829 * t317 + 48.0 * t44603 - 72.0 * t44262 - t788 * (t44736 + t44751 + t44767 + t44781) * t317 - 2.0 * t44131 - 48.0 * t44352 + 48.0 * t44362;
    (t44789,)
}
