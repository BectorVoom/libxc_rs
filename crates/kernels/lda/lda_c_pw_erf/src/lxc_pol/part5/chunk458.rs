//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 458/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk458<F: Float>(t2125: F, t2129: F, t2133: F, t2136: F, t2139: F, t2142: F, t2145: F, t2148: F, t2150: F, t2155: F, t2160: F, t2165: F, t2170: F, t2173: F, t2175: F, t2180: F, t2185: F, t2190: F, t2195: F) -> (F,) {
    let t2290 = -t2125 + t2129 + t2133 - t2136 + t2139 + t2142 + t2145 + t2148 + t2150 - t2155 - t2160 + t2165 - t2170 + t2173 + t2175 + t2180 - t2185 + t2190 - t2195;
    (t2290,)
}
