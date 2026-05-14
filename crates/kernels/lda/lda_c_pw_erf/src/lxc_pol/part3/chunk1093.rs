//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1093/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1093<F: Float>(t12001: F, t12003: F, t12008: F, t12010: F, t12012: F, t12014: F, t12016: F, t12018: F, t12020: F, t12022: F, t12024: F, t12028: F, t12034: F, t12039: F, t12040: F, t12041: F, t12042: F, t12043: F, t12045: F, t12047: F, t12049: F, t12051: F, t12053: F, t12055: F, t12059: F, t12063: F) -> (F, F) {
    let t14986 = t12001 + t12003 + t12008 - t12010 + t12012 - t12014 - t12016 - t12018 + t12020 + t12022 + t12024 - t12028 + t12034;
    let t14987 = t12039 + t12040 + t12041 + t12042 + t12043 - t12045 - t12047 - t12049 + t12051 - t12053 - t12055 + t12059 + t12063;
    (t14986, t14987)
}
