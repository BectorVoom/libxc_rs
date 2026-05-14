//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1107/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1107<F: Float>(t2260: F, t3936: F, t5788: F, t656: F, t12955: F, t12959: F, t12962: F, t12966: F, t12971: F, t12975: F, t12979: F, t12982: F, t12985: F, t12988: F, t12991: F, t12996: F, t12999: F, t13001: F, t13009: F, t13011: F, t13013: F, t13016: F, t13018: F, t13022: F, t13025: F, t13028: F, t13031: F, t13033: F) -> (F, F) {
    let t15060 = t2260 * t3936;
    let t15062 = t5788 * t656;
    let t15064 = t12955 + t12959 + t12962 - t12966 + t12971 - 0.013506172839506173 * t15060 + 2.0 / 3.0 * t15062 - t12975 + t12979 - t12982 + t12985 + t12988 + t12991;
    let t15065 = -t12996 + t12999 + t13001 + t13009 + t13011 - t13013 + t13016 - t13018 - t13022 - t13025 - t13028 - t13031 - t13033;
    (t15064, t15065)
}
