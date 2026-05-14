//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1376/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1376<F: Float>(t10245: F, t745: F, t10265: F, t170: F, t60: F, t22210: F, t236: F, t26608: F, t26610: F, t26611: F, t26613: F, t26615: F, t26623: F, t26625: F, t26627: F, t26630: F, t32195: F, t596: F, t76: F) -> (F,) {
    let t33575 = t10245 * t745;
    let t33580 = t60 * t10265 * t170;
    let t33583 = t26608 + t26610 + 0.15584273195113317383e3 * t26611 + 0.15584273195113317383e3 * t26613 + 0.51947577317044391277e2 * t26615 + t26623 + 0.5848223622634646207e0 * t32195 * t76 * t236 + 0.5848223622634646207e0 * t33575 + 0.19518446340543131715e0 * t26625 - 0.28895839882605942647e1 * t26627 - t22210 - t26630 - 0.675260332e-1 * t596 * t33580;
    (t33583,)
}
