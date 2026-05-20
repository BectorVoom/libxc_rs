//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1621/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1621<F: Float>(t3566: F, t3766: F, t5330: F, t3568: F, t3601: F, t12646: F, t12915: F, t247: F, t5384: F, t12831: F, t12865: F, t1260: F, t12889: F) -> (F, F, F, F, F) {
    let t44550 = t3566 * t3766;
    let t44551 = t44550 * t5330;
    let t44552 = t3568 * t3601;
    let t44559 = t5384 * t247 * t12915 * t12646;
    let t44561 = t12831 * t12865;
    let t44568 = t12889 * t1260;
    (t44551, t44552, t44559, t44561, t44568)
}
