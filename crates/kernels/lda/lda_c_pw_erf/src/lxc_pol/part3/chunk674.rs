//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 674/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk674<F: Float>(t242: F, t4100: F, t1198: F, t632: F, t1143: F, t458: F, t2853: F, t41: F, t1203: F, t1155: F, t153: F, t156: F, t168: F, t245: F, t3196: F, t3373: F, t3375: F, t3378: F, t4079: F, t4084: F, t4087: F, t4091: F, t4092: F, t4095: F, t4096: F, t4099: F) -> (F, F, F, F, F, F, F) {
    let t4101 = t4100 * t242;
    let t4103 = t1198 * t632;
    let t4106 = F::new(0.2512884616065132) * t458 * t1143;
    let t4107 = t41 * t2853;
    let t4110 = t1203 * t632;
    let t4113 = F::new(0.5025769232130264) * t1155 * t242;
    let t4114 = F::new(0.42708890021612717) * t153 * t156 * t3196 - t3373 - F::new(1.7083556008645087) * t3375 + F::new(3.9861630686838536) * t3378 - F::new(0.011938374665504766) * t168 * t245 * t4079 - F::new(0.15917832887339686) * t4084 + F::new(0.05969187332752383) * t4087 + t4091 - F::new(0.2512884616065132) * t4092 - t4095 - F::new(0.5025769232130264) * t4096 - t4099 + F::new(0.2512884616065132) * t4101 + F::new(0.5025769232130264) * t4103 + t4106 - F::new(0.0837628205355044) * t4107 * t242 - F::new(0.2512884616065132) * t4110 + t4113;
    (t4101, t4103, t4106, t4107, t4110, t4113, t4114)
}
