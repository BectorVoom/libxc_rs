//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 239/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk239(t721: f64, t984: f64, t570: f64, t574: f64, t666: f64, t670: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t676: f64, t681: f64, t687: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t986 = 3.7610742193750633_f64 * t984 * t721;
    let t993 = 0.008222864943561326_f64 * t570;
    let t994 = 0.09983749558483038_f64 * t574;
    let t995 = 2.2984542076810275_f64 * t666;
    let t996 = 1.532302805120685_f64 * t670;
    let t1000 = 0.15282509383508946_f64 * t612;
    let t1001 = 0.10188339589005964_f64 * t616;
    let t1005 = t995 + t996 + 2.2984542076810275_f64 * t676 + 2.2984542076810275_f64 * t681 - 2.2984542076810275_f64 * t687 + t1000 + t1001 + 0.15282509383508946_f64 * t626 + 0.15282509383508946_f64 * t636 - 0.15282509383508946_f64 * t653;
    (t986, t993, t994, t995, t996, t1000, t1001, t1005)
}
