//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1102/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1102<F: Float>(t2146: F, t3825: F, t12870: F, t12873: F, t12876: F, t12878: F, t12880: F, t12883: F, t12885: F, t12887: F, t12889: F, t12891: F, t12893: F, t12895: F) -> (F, F) {
    let t12897 = F::new(4.0) / F::new(15.0) * t2146 * t3825;
    let t12898 = -t12870 - t12873 + t12876 + t12878 + t12880 + t12883 + t12885 - t12887 + t12889 - t12891 + t12893 + t12895 - t12897;
    (t12897, t12898)
}
