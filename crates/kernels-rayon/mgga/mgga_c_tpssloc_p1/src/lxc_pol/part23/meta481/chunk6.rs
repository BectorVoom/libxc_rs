//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1445/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1445(t50846: f64, t63888: f64, t63893: f64, t63911: f64, t71335: f64, t71337: f64, t71408: f64, t77959: f64, t77963: f64, t77967: f64, t78084: f64, t44466: f64, t71470: f64, t71472: f64, t71474: f64, t77971: f64, t77975: f64, t77979: f64, t77983: f64, t78087: f64, t78090: f64, t78093: f64, t78100: f64) -> (f64, f64) {
    let t78596 = 10.0_f64 / 27.0_f64 * t63888 - 20.0_f64 / 9.0_f64 * t63893 - 4.0_f64 / 9.0_f64 * t71335 + 8.0_f64 / 3.0_f64 * t71337 + 160.0_f64 / 81.0_f64 * t50846 - 8.0_f64 / 9.0_f64 * t77959 + 14.0_f64 / 81.0_f64 * t77963 - 10.0_f64 / 9.0_f64 * t63911 + 4.0_f64 / 9.0_f64 * t71408 + t77967 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t78084;
    let t78607 = 2.0_f64 * t78087 - t77971 - 4.0_f64 / 3.0_f64 * t78090 - 6.0_f64 * t78093 + 2.0_f64 * t77975 - 4.0_f64 * t77979 - t77983 / 6.0_f64 - t44466 + 16.0_f64 / 81.0_f64 * t71470 - 4.0_f64 / 9.0_f64 * t78100 - 8.0_f64 / 9.0_f64 * t71472 + 8.0_f64 / 3.0_f64 * t71474;
    (t78596, t78607)
}
