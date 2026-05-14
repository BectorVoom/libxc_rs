//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1416/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1416<F: Float>(t2124: F, t24665: F, t2557: F, t2562: F, t2591: F, t25951: F, t25979: F, t30381: F, t30394: F, t33244: F, t34339: F, t34343: F, t34347: F, t34351: F, t34354: F, t34357: F, t360: F, t7512: F, t9098: F, t9103: F) -> (F,) {
    let t34373 = 0.1047928639570397803e0 * t34339 + 0.52396431978519890152e-1 * t34343 + 0.32927245914677557992e-1 * t34347 - 0.24451668256642615404e1 * t30381 + 0.29272321618148349055e-1 * t34351 + 0.52396431978519890152e-1 * t34354 + 0.29272321618148349055e-1 * t34357 - 0.98781737744032673979e-1 * t25951 + 0.38415120233790484324e0 * t2557 * t2124 * t33244 * t2591 + 0.69345773920434148506e0 * t30394 - 0.78013995660488417067e0 * t24665 * t360 * t2562 * t9098 - 0.78013995660488417067e0 * t7512 * t360 * t2562 * t9103 - t25979;
    (t34373,)
}
