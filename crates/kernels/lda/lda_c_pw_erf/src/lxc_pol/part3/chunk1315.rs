//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1315/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1315<F: Float>(t5791: F, t656: F, t3912: F, t5795: F, t5798: F, t2260: F, t3915: F, t1217: F, t2281: F, t3704: F, t858: F, t14108: F, t14112: F, t14157: F, t14162: F, t14164: F, t14166: F, t14170: F) -> F {
    let t15143 = t5791 * t656;
    let t15144 = F::new(4.0) / F::new(3.0) * t15143;
    let t15145 = t5795 * t3912;
    let t15146 = F::new(2e-21) * t15145;
    let t15147 = t5798 * t656;
    let t15149 = t2260 * t3915;
    let t15150 = F::new(2e-21) * t15149;
    let t15151 = t2281 * t1217;
    let t15152 = F::new(2.0) / F::new(45.0) * t15151;
    let t15153 = t858 * t3704;
    let t15155 = t15144 + t15146 + F::new(2.0) / F::new(3.0) * t15147 + t15150 + t14108 + t14112 + t15152 - F::new(8.0) / F::new(405.0) * t15153 + t14157 - t14162 + t14164 + t14166 + t14170;
    t15155
}
