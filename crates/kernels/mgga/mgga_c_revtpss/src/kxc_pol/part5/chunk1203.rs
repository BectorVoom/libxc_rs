//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1203/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1203<F: Float>(t19077: F, t291: F, t4719: F, t4734: F, t6226: F, t974: F, t981: F, t15170: F, t15189: F, t15447: F, t15457: F, t15459: F, t18944: F, t18961: F, t18964: F, t18967: F, t18970: F, t18973: F) -> (F, F, F, F) {
    let t19079 = F::new(0.621814e-1) * t19077 * t291;
    let t19081 = F::new(0.34631718211362927517e2) * t4719 * t4734;
    let t19082 = t6226 * t974;
    let t19084 = F::new(0.35089341735807877242e1) * t981 * t19082;
    let t19103 = F::new(0.59793333333333333334e0) * t18944 + F::new(0.16431333333333333333e0) * t18961 - F::new(0.54771111111111111112e-1) * t18964 - F::new(0.36514074074074074075e-1) * t18967 - F::new(0.49293999999999999999e0) * t18970 + F::new(0.32862666666666666666e0) * t18973 - t15447 + F::new(0.36514074074074074073e-1) * t15170 - F::new(0.26574814814814814815e0) * t15189 + t15457 + t15459;
    (t19079, t19081, t19084, t19103)
}
