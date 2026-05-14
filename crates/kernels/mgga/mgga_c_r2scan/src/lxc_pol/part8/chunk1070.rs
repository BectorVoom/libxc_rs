//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1070/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1070<F: Float>(t18956: F, t273: F, t5421: F, t625: F, t409: F, t4743: F, t1401: F, t18950: F, t18953: F, t392: F, t1266: F, t22: F, t6: F, t18946: F, t18948: F, t18951: F, t18954: F, t384: F, t401: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18957 = 0.31003950617283950618e1 * t18956;
    let t18959 = t5421 * t273 * t625;
    let t18960 = 0.68258333333333333335e-1 * t18959;
    let t18961 = t4743 * t409;
    let t18962 = 0.10921333333333333333e1 * t18961;
    let t18963 = t1401 * t18950;
    let t18964 = 0.12134814814814814815e1 * t18963;
    let t18965 = t392 * t18953;
    let t18966 = 0.10617962962962962963e1 * t18965;
    let t18968 = t22 * t6 * t1266;
    let t18969 = 0.13388493827160493828e1 * t18968;
    let t18973 = 1.0 * t384 * (-0.21099166666666666667e1 * t18946 + 0.202552e2 * t18948 - 0.75019259259259259258e1 * t18951 + 0.6564185185185185185e1 * t18954 + t18957 + t18960 - t18962 + t18964 + t18966 + t18969) * t401;
    (t18957, t18959, t18960, t18961, t18962, t18963, t18964, t18965, t18966, t18968, t18969, t18973)
}
