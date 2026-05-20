//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1471/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1471<F: Float>(t11501: F, t3014: F, t2876: F, t2918: F, t2924: F, t11385: F, t11387: F, t2875: F, t11112: F, t11528: F, t11116: F, t11294: F) -> (F, F, F, F, F) {
    let t41832 = t11501 * t3014;
    let t41841 = F::new(36.0) * t2924 * t2876 * t2918;
    let t41845 = F::cast_from(0.3103560775156404018e4_f64) * t11385 * t2875 * t11387 * t2918;
    let t41847 = F::new(24.0) * t11528 * t11112;
    let t41849 = F::cast_from(0.1929837539843104208e3_f64) * t11294 * t11116;
    (t41832, t41841, t41845, t41847, t41849)
}
