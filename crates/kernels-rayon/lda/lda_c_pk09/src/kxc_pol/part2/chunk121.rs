//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 121/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk121(t280: f64, t359: f64, t322: f64, t305: f64) -> (f64, f64, f64, f64) {
    let t360 = t359 * t280;
    let t363 = t322 + 0.4822571819944727_f64;
    let t364 = f64::ln(t363);
    let t365 = t364 * t305;
    (t360, t363, t364, t365)
}
